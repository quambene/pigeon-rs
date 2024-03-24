use crate::{
    arg::{self},
    email_builder::{Confirmed, Email},
    email_formatter::EmlFormatter,
    email_transmission::Client,
    helper::format_green,
};
use anyhow::Result;
use clap::ArgMatches;

pub fn send(matches: &ArgMatches) -> Result<(), anyhow::Error> {
    if matches.is_present(arg::VERBOSE) {
        println!("matches: {:#?}", matches);
    }

    let email = Email::build(matches)?;

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
