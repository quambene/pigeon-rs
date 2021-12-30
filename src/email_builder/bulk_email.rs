use super::{MimeFormat, Receiver};
use crate::{
    arg, cmd,
    data_loader::TabularData,
    data_sources::{query_postgres, read_csv},
    email_builder::{Confirmed, Email, Message, MessageTemplate},
    email_formatter::EmlFormatter,
    email_transmission::Client,
};
use anyhow::{anyhow, Context, Result};
use clap::{ArgMatches, Values};
use polars::prelude::DataFrame;
use std::{io, path::PathBuf};

#[derive(Debug)]
pub struct BulkEmail<'a> {
    pub emails: Vec<Email<'a>>,
}

impl<'a> BulkEmail<'a> {
    pub fn new(matches: &'a ArgMatches<'a>) -> Result<Self, anyhow::Error> {
        let sender = if matches.is_present(arg::SENDER) {
            match matches.value_of(arg::SENDER) {
                Some(sender) => sender,
                None => return Err(anyhow!("Missing value for argument '{}'", arg::SENDER)),
            }
        } else {
            return Err(anyhow!("Missing argument '{}'", arg::SENDER));
        };

        let df_receiver = match (
            matches.is_present(arg::RECEIVER_QUERY),
            matches.is_present(arg::RECEIVER_FILE),
        ) {
            (true, false) => Self::dataframe_from_query(matches)?,
            (false, true) => Self::dataframe_from_file(matches)?,
            (true, true) => {
                return Err(anyhow!(
                    "Argument conflict: arguments {} and {} are not allowed at the same time. Check usage via '{} help {}'",
                    arg::RECEIVER_QUERY,
                    arg::RECEIVER_FILE,
                    cmd::BIN,
                    cmd::SEND_BULK,
                ))
            }
            (false, false) => {
                return Err(anyhow!(
                    "Missing arguments: please specify argument {} or {}. Check usage via '{} help {}'",
                    arg::RECEIVER_QUERY,
                    arg::RECEIVER_FILE,
                    cmd::BIN,
                    cmd::SEND_BULK,
                ))
            }
        };

        let emails = if matches.is_present(arg::PERSONALIZE) {
            match matches.values_of(arg::PERSONALIZE) {
                Some(personalized_columns) => {
                    let message_template = MessageTemplate::read(matches)?;
                    let default_message = Message::default(&message_template)?;
                    Self::create_personalized_emails(
                        matches,
                        sender,
                        df_receiver,
                        personalized_columns,
                        &default_message,
                    )?
                }
                None => return Err(anyhow!("Missing value for argument '{}'", arg::PERSONALIZE)),
            }
        } else {
            let message_template = MessageTemplate::read(matches)?;
            let message = Message::default(&message_template)?;
            Self::create_emails(matches, sender, df_receiver, &message)?
        };

        Ok(BulkEmail { emails })
    }

    pub fn process(&self, matches: &ArgMatches<'_>) -> Result<(), anyhow::Error> {
        let client = Client::new()?;
        let eml_formatter = EmlFormatter::new(matches)?;

        println!("Sending email to {} receivers ...", self.emails.len());

        for email in &self.emails {
            let sent_email = client.send(matches, email)?;
            sent_email.display_status();
            eml_formatter.archive(matches, email)?;
        }

        if matches.is_present(arg::DRY_RUN) {
            println!("All emails sent (dry run).");
        } else {
            println!("All emails sent.");
        }

        Ok(())
    }

    pub fn confirm(&self) -> Result<Confirmed, anyhow::Error> {
        let mut input = String::new();
        let email_count = self.emails.len();
        let receivers: Vec<String> = self
            .emails
            .iter()
            .map(|email| email.receiver.to_string())
            .collect();
        println!(
            "Preparing to send an email to {} recipients: {:#?}",
            email_count, receivers
        );

        println!(
            "Should an email be sent to {} recipients? Yes (y) or no (n)",
            email_count
        );
        let confirmation = loop {
            io::stdin()
                .read_line(&mut input)
                .context("Can't read input")?;
            match input.trim() {
                "y" | "yes" | "Yes" => {
                    break Confirmed::Yes;
                }
                "n" | "no" | "No" => {
                    println!("Aborted ...");
                    break Confirmed::No;
                }
                _ => {
                    println!("Choose yes (y) or no (n). Try again.");
                    continue;
                }
            }
        };
        Ok(confirmation)
    }

    fn dataframe_from_query(matches: &ArgMatches<'_>) -> Result<DataFrame, anyhow::Error> {
        let receiver_query = Receiver::query(matches)?;
        let df_receiver = query_postgres(matches, receiver_query)?;

        if matches.is_present(arg::DISPLAY) {
            println!("Display query result: {}", df_receiver);
        }

        Ok(df_receiver)
    }

    fn dataframe_from_file(matches: &ArgMatches<'_>) -> Result<DataFrame, anyhow::Error> {
        let receiver_file = Receiver::file_name(matches)?;
        let path = PathBuf::from(receiver_file);
        let df_receiver = read_csv(&path)?;

        if matches.is_present(arg::DISPLAY) {
            println!("Display csv file: {}", df_receiver);
        }

        Ok(df_receiver)
    }

    fn create_emails(
        matches: &ArgMatches<'_>,
        sender: &'a str,
        df_receiver: DataFrame,
        message: &Message,
    ) -> Result<Vec<Email<'a>>, anyhow::Error> {
        let mut emails: Vec<Email> = vec![];
        let receiver_column_name = Receiver::column_name(matches)?;
        let receivers = TabularData::column(receiver_column_name, &df_receiver)?;

        for receiver in receivers {
            match receiver {
                Some(receiver) => {
                    let mime_format = MimeFormat::new(matches, sender, receiver, &message)?;
                    emails.push(Email {
                        sender,
                        receiver: receiver.to_string(),
                        message: message.to_owned(),
                        mime_format,
                    });
                }
                None => continue,
            }
        }

        Ok(emails)
    }

    fn create_personalized_emails(
        matches: &ArgMatches<'_>,
        sender: &'a str,
        df_receiver: DataFrame,
        personalized_columns: Values,
        default_message: &Message,
    ) -> Result<Vec<Email<'a>>, anyhow::Error> {
        let mut emails: Vec<Email> = vec![];
        let columns: Vec<&str> = personalized_columns.collect();
        let receiver_column_name = Receiver::column_name(matches)?;

        for i in 0..df_receiver.height() {
            let mut message = default_message.clone();

            for &col_name in columns.iter() {
                let col_value = TabularData::row(i, col_name, &df_receiver)?;
                message = message.personalize(col_name, col_value)?;
            }

            let receiver = TabularData::row(i, receiver_column_name, &df_receiver)?;
            let mime_format = MimeFormat::new(matches, sender, receiver, &message)?;

            emails.push(Email {
                sender,
                receiver: receiver.to_string(),
                message,
                mime_format,
            });
        }

        Ok(emails)
    }
}
