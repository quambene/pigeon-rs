use crate::{
    arg,
    email_builder::{Confirmed, Email, Message, MimeFormat, Receiver, Sender},
    email_formatter::EmlFormatter,
    email_transmission::Client,
    helper::format_green,
};
use anyhow::Result;
use clap::ArgMatches;
use std::time::SystemTime;

pub fn send(matches: &ArgMatches) -> Result<(), anyhow::Error> {
    if matches.is_present(arg::VERBOSE) {
        println!("matches: {:#?}", matches);
    }

    let now = SystemTime::now();
    let sender = Sender::init(matches)?;
    let receiver = Receiver::init(matches)?;
    let message = Message::build(matches)?;
    let attachment = matches.value_of(arg::ATTACHMENT);
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
        let confirmation = email.confirm(matches)?;
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
