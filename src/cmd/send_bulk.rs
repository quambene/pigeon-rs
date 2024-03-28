use crate::{
    arg,
    email_builder::{BulkEmail, Confirmed, Email, Message, Receiver, Sender},
    email_formatter::EmlFormatter,
    email_transmission::Client,
    helper::format_green,
};
use anyhow::Result;
use anyhow::{anyhow, Context};
use clap::ArgMatches;
use std::{io, path::Path};

pub fn send_bulk(matches: &ArgMatches) -> Result<(), anyhow::Error> {
    if matches.is_present(arg::VERBOSE) {
        println!("matches: {:#?}", matches);
    }

    let sender = Sender::from_args(matches)?;
    let receiver_column_name = Receiver::column_name(matches)?;
    let df_receiver = Receiver::dataframe(matches)?;
    let default_message = Message::from_args(matches)?;
    let attachment = matches.value_of(arg::ATTACHMENT).map(Path::new);
    let bulk_email = if matches.is_present(arg::PERSONALIZE) {
        if let Some(personalized_columns) = matches.values_of(arg::PERSONALIZE) {
            BulkEmail::personalize(
                sender,
                receiver_column_name,
                &df_receiver,
                &default_message,
                personalized_columns,
                attachment,
            )?
        } else {
            return Err(anyhow!("Missing value for argument '{}'", arg::PERSONALIZE));
        }
    } else {
        BulkEmail::new(
            sender,
            receiver_column_name,
            &df_receiver,
            &default_message,
            attachment,
        )?
    };

    if matches.is_present(arg::DISPLAY) {
        println!("Display emails: {:#?}", bulk_email);
    }

    if matches.is_present(arg::DRY_RUN) {
        println!("Dry run: {}", format_green("activated"));
    }

    if matches.is_present(arg::ASSUME_YES) {
        process_emails(&bulk_email.emails, matches)?;
    } else {
        let confirmation = confirm_emails(&bulk_email.emails)?;
        match confirmation {
            Confirmed::Yes => {
                process_emails(&bulk_email.emails, matches)?;
            }
            Confirmed::No => (),
        }
    }

    Ok(())
}

pub fn process_emails(emails: &[Email], matches: &ArgMatches) -> Result<(), anyhow::Error> {
    let client = Client::init(matches)?;
    let eml_formatter = EmlFormatter::new(matches)?;

    println!("Sending email to {} receivers ...", emails.len());

    for email in emails {
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

pub fn confirm_emails(emails: &[Email]) -> Result<Confirmed, anyhow::Error> {
    let mut input = String::new();
    let email_count = emails.len();
    let receivers = emails
        .iter()
        .map(|email| email.receiver.as_str())
        .collect::<Vec<_>>();
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
