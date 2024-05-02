use crate::{
    arg,
    email_builder::{Confirmed, Email, Message, MimeFormat, Receiver, Sender},
    email_formatter::EmlFormatter,
    email_transmission::Client,
    utils::format_green,
};
use anyhow::Context;
use chrono::Utc;
use clap::ArgMatches;
use std::{io, path::Path, time::SystemTime};

pub fn send(matches: &ArgMatches) -> Result<(), anyhow::Error> {
    if matches.get_flag(arg::VERBOSE) {
        println!("matches: {:#?}", matches);
    }

    let now = SystemTime::now();
    let dry_run = matches.get_flag(arg::DRY_RUN);
    let is_archived = matches.get_flag(arg::ARCHIVE);
    let archive_dir = Path::new(arg::value(arg::ARCHIVE_DIR, matches)?);
    let sender = Sender(arg::value(arg::SENDER, matches)?);
    let receiver = Receiver(arg::value(arg::RECEIVER, matches)?);
    let message = Message::from_args(matches)?;
    let attachment = matches.get_one::<String>(arg::ATTACHMENT).map(Path::new);
    let mime_format = MimeFormat::new(sender, receiver, &message, attachment, now)?;
    let email = Email::new(sender, receiver, &message, &mime_format)?;

    if matches.get_flag(arg::DISPLAY) {
        println!("Display email: {:#?}", email);
    }

    if dry_run {
        println!("Dry run: {}", format_green("activated"));
    }

    let now = Utc::now();
    let client = Client::from_args(matches)?;
    let eml_formatter = EmlFormatter::new(archive_dir)?;

    println!("Sending email to 1 receiver ...");

    if matches.get_flag(arg::ASSUME_YES) {
        let sent_email = client.send(&email)?;
        sent_email.display_status();

        if is_archived {
            eml_formatter.archive(&email, now, dry_run)?;
        }
    } else {
        let confirmation = confirm_email(&email)?;
        match confirmation {
            Confirmed::Yes => {
                let sent_email = client.send(&email)?;
                sent_email.display_status();

                if is_archived {
                    eml_formatter.archive(&email, now, dry_run)?;
                }
            }
            Confirmed::No => (),
        }
    }

    if matches.get_flag(arg::DRY_RUN) {
        println!("Email sent (dry run)");
    } else {
        println!("Email sent");
    }

    Ok(())
}

pub fn confirm_email(email: &Email) -> Result<Confirmed, anyhow::Error> {
    let mut input = String::new();

    println!(
        "Preparing to send an email to 1 recipient: {}",
        email.receiver.0
    );

    println!("Should an email be sent to 1 recipient? Yes (y) or no (n)");
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
