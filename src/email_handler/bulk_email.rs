use crate::{
    arg, cmd,
    data_sources::{query_postgres, read_csv},
    email_handler::{Email, Message, MessageTemplate},
    email_provider,
    helper::{check_send_status, format_green},
};
use anyhow::{anyhow, Context, Result};
use clap::{ArgMatches, Values};
use polars::prelude::{DataFrame, TakeRandom};
use std::{io, path::PathBuf};

#[derive(Debug)]
pub struct BulkEmail {
    pub emails: Vec<Email>,
}

impl BulkEmail {
    pub fn new(matches: &ArgMatches<'_>) -> Result<Self, anyhow::Error> {
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
            (true, false) => dataframe_from_query(matches)?,
            (false, true) => dataframe_from_file(matches)?,
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
                Some(values) => create_personalized_emails(matches, sender, df_receiver, values)?,
                None => return Err(anyhow!("Missing value for argument '{}'", arg::PERSONALIZE)),
            }
        } else {
            create_emails(matches, sender, df_receiver)?
        };

        Ok(BulkEmail { emails })
    }

    pub fn send(&self, matches: &ArgMatches<'_>) -> Result<(), anyhow::Error> {
        if matches.is_present(arg::DRY_RUN) {
            // Setup client but do not send email
            let _client = email_provider::setup_ses_client(matches)?;

            for email in &self.emails {
                println!("{} ... {}", email.receiver, format_green("dry run"));
            }

            println!("All emails sent (dry run).");
            Ok(())
        } else {
            let client = email_provider::setup_ses_client(matches)?;

            for email in &self.emails {
                let res = email_provider::send_email(&email, &client);
                let status = check_send_status(res);
                println!("{:#?} ... {}", email.receiver, status);
            }

            println!("All emails sent.");
            Ok(())
        }
    }

    pub fn confirm_and_send(&self, matches: &ArgMatches<'_>) -> Result<(), anyhow::Error> {
        let mut input = String::new();
        let email_count = self.emails.len();
        let receivers: Vec<String> = self
            .emails
            .iter()
            .map(|el| el.receiver.to_string())
            .collect();
        println!(
            "Preparing to send an email to {} recipients: {:#?}",
            email_count, receivers
        );

        println!("Should an email be sent to all recipients? Yes (y) or no (n)");
        loop {
            io::stdin().read_line(&mut input).expect("Can't read input");
            match input.trim() {
                "y" | "yes" | "Yes" => {
                    self.send(matches)?;
                    break;
                }
                "n" | "no" | "No" => {
                    println!("Aborted ...");
                    break;
                }
                _ => {
                    println!("Choose yes (y) or no (n). Try again.");
                    continue;
                }
            }
        }
        Ok(())
    }

    pub fn archive(&self, matches: &ArgMatches<'_>) -> Result<(), anyhow::Error> {
        if matches.is_present(arg::MESSAGE_FILE) {
            MessageTemplate::archive(matches)?;
        }

        Ok(())
    }
}

fn dataframe_from_query(matches: &ArgMatches<'_>) -> Result<DataFrame, anyhow::Error> {
    let receiver_query = match matches.value_of(arg::RECEIVER_QUERY) {
        Some(receiver_query) => receiver_query,
        None => {
            return Err(anyhow!(
                "Missing value for argument '{}'",
                arg::RECEIVER_QUERY
            ))
        }
    };

    let df_receiver = query_postgres(matches, receiver_query)?;
    Ok(df_receiver)
}

fn dataframe_from_file(matches: &ArgMatches<'_>) -> Result<DataFrame, anyhow::Error> {
    let receiver_file = match matches.value_of(arg::RECEIVER_FILE) {
        Some(receiver_file) => receiver_file,
        None => {
            return Err(anyhow!(
                "Missing value for argument '{}'",
                arg::RECEIVER_FILE
            ))
        }
    };

    let path = PathBuf::from(receiver_file);
    let df_receiver = read_csv(matches, &path)?;

    Ok(df_receiver)
}

fn create_emails(
    matches: &ArgMatches<'_>,
    sender: &str,
    df_receiver: DataFrame,
) -> Result<Vec<Email>, anyhow::Error> {
    let receiver_col: &str;

    // If argument 'RECEIVER_COLUMN' is not present the default value 'email' will be used
    match matches.value_of(arg::RECEIVER_COLUMN) {
        Some(col_name) => receiver_col = col_name,
        None => {
            return Err(anyhow!(
                "Missing value for argument '{}'",
                arg::RECEIVER_COLUMN
            ))
        }
    };

    let message_template = MessageTemplate::read(matches)?;
    let message = &Message::default(&message_template)?;

    let mut emails: Vec<Email> = vec![];
    let email_series = df_receiver.column(receiver_col)?;
    let chunked_array = email_series
        .utf8()
        .context("Can't convert series to chunked array")?;

    for email_opt in chunked_array {
        match email_opt {
            Some(email) => {
                emails.push(Email {
                    sender: sender.to_string(),
                    receiver: email.to_string(),
                    message: message.clone(),
                });
            }
            None => continue,
        }
    }

    Ok(emails)
}

fn create_personalized_emails(
    matches: &ArgMatches<'_>,
    sender: &str,
    df_receiver: DataFrame,
    values: Values,
) -> Result<Vec<Email>, anyhow::Error> {
    let message_template = MessageTemplate::read(matches)?;
    let default_message = &Message::default(&message_template)?;

    let mut emails: Vec<Email> = vec![];

    for i in 0..df_receiver.height() {
        let mut message = default_message.clone();
        let personalized_columns = values.clone();

        for col_name in personalized_columns {
            match df_receiver.column(col_name)?.utf8()?.get(i) {
                Some(col_value) => message = message.personalize(col_name, col_value)?,
                None => {
                    return Err(anyhow!(
                        "Missing value for column '{}' in row {}",
                        col_name,
                        i
                    ))
                }
            };
        }

        let receiver_col: &str;

        // If argument 'RECEIVER_COLUMN' is not present the default value 'email' will be used
        match matches.value_of(arg::RECEIVER_COLUMN) {
            Some(col_name) => receiver_col = col_name,
            None => {
                return Err(anyhow!(
                    "Missing value for argument '{}'",
                    arg::RECEIVER_COLUMN
                ))
            }
        };

        let receiver = df_receiver
            .column(receiver_col)
            .context(format!(
                "Invalid value for argument '{}'",
                arg::RECEIVER_COLUMN
            ))?
            .utf8()?
            .get(i)
            .unwrap()
            .to_string();

        emails.push(Email {
            sender: sender.to_string(),
            receiver,
            message,
        });
    }

    Ok(emails)
}
