use crate::{
    arg,
    email_builder::{Confirmed, Email, Message, MimeFormat, Receiver, Sender},
    email_formatter::EmlFormatter,
    email_transmission::Client,
    helper::format_green,
};
use anyhow::Context;
use clap::ArgMatches;
use std::{io, path::Path, time::SystemTime};

pub fn send(matches: &ArgMatches) -> Result<(), anyhow::Error> {
    if matches.is_present(arg::VERBOSE) {
        println!("matches: {:#?}", matches);
    }

    let now = SystemTime::now();
    let sender = Sender::from_args(matches)?;
    let receiver = Receiver::from_args(matches)?;
    let message = Message::from_args(matches)?;
    let attachment = matches.value_of(arg::ATTACHMENT).map(Path::new);
    let mime_format = MimeFormat::new(sender, receiver, &message, attachment, now)?;
    let email = Email::new(sender, receiver, &message, &mime_format)?;

    if matches.is_present(arg::DISPLAY) {
        println!("Display email: {:#?}", email);
    }

    if matches.is_present(arg::DRY_RUN) {
        println!("Dry run: {}", format_green("activated"));
    }

    let client = Client::init(matches)?;
    let eml_formatter = EmlFormatter::new(matches)?;

    println!("Sending email to 1 recipient ...");

    if matches.is_present(arg::ASSUME_YES) {
        let sent_email = client.send(matches, &email)?;
        sent_email.display_status();
        eml_formatter.archive(matches, &email)?;
    } else {
        let confirmation = confirm_email(&email)?;
        match confirmation {
            Confirmed::Yes => {
                let sent_email = client.send(matches, &email)?;
                sent_email.display_status();
                eml_formatter.archive(matches, &email)?;
            }
            Confirmed::No => (),
        }
    }

    if matches.is_present(arg::DRY_RUN) {
        println!("Email sent (dry run).");
    } else {
        println!("Email sent.");
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
