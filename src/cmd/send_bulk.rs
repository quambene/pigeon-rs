use crate::{
    arg,
    email_builder::{BulkEmail, BulkReceiver, Confirmed, Email, Message, Sender},
    email_formatter::EmlFormatter,
    email_transmission::Client,
    utils::format_green,
};
use anyhow::{anyhow, Context, Result};
use chrono::Utc;
use clap::ArgMatches;
use std::{io, path::Path};

pub fn send_bulk(matches: &ArgMatches) -> Result<(), anyhow::Error> {
    if matches.is_present(arg::VERBOSE) {
        println!("matches: {:#?}", matches);
    }

    let dry_run = matches.is_present(arg::DRY_RUN);
    let is_archived = matches.is_present(arg::ARCHIVE);
    let archive_dir = Path::new(arg::value(arg::ARCHIVE_DIR, matches)?);
    let sender = Sender(arg::value(arg::SENDER, matches)?);
    let receivers = BulkReceiver::from_args(matches)?;
    let message = Message::from_args(matches)?;
    let attachment = matches.value_of(arg::ATTACHMENT).map(Path::new);

    let bulk_email = if matches.is_present(arg::PERSONALIZE) {
        if let Some(personalized_columns) = matches.values_of(arg::PERSONALIZE) {
            let personalized_columns = personalized_columns.collect::<Vec<&str>>();
            BulkEmail::new(
                sender,
                &receivers,
                &message,
                attachment,
                &personalized_columns,
            )?
        } else {
            return Err(anyhow!("Missing value for argument '{}'", arg::PERSONALIZE));
        }
    } else {
        BulkEmail::new(sender, &receivers, &message, attachment, &[])?
    };
    let client = Client::from_args(matches)?;
    let eml_formatter = EmlFormatter::new(archive_dir)?;

    if matches.is_present(arg::DISPLAY) {
        println!("Display emails: {:#?}", bulk_email);
    }

    if dry_run {
        println!("Dry run: {}", format_green("activated"));
    }

    if matches.is_present(arg::ASSUME_YES) {
        process_emails(
            &client,
            &eml_formatter,
            &bulk_email.emails,
            dry_run,
            is_archived,
        )?;
    } else {
        let confirmation = confirm_emails(&bulk_email.emails)?;
        match confirmation {
            Confirmed::Yes => {
                process_emails(
                    &client,
                    &eml_formatter,
                    &bulk_email.emails,
                    dry_run,
                    is_archived,
                )?;
            }
            Confirmed::No => (),
        }
    }

    if dry_run {
        println!("All emails sent (dry run)");
    } else {
        println!("All emails sent");
    }

    Ok(())
}

pub fn process_emails<'a>(
    client: &Client<'a>,
    eml_formatter: &EmlFormatter,
    emails: &'a [Email],
    dry_run: bool,
    is_archived: bool,
) -> Result<(), anyhow::Error> {
    println!("Sending email to {} receivers ...", emails.len());

    for email in emails {
        let sent_email = client.send(email)?;
        sent_email.display_status();

        if is_archived {
            let now = Utc::now();
            eml_formatter.archive(email, now, dry_run)?;
        }
    }

    Ok(())
}

pub fn confirm_emails(emails: &[Email]) -> Result<Confirmed, anyhow::Error> {
    let mut input = String::new();
    let email_count = emails.len();
    let receivers = emails
        .iter()
        .map(|email| email.receiver.as_ref())
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
