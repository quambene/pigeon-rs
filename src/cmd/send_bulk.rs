use crate::{
    arg::{self},
    email_builder::{BulkEmail, Confirmed, Message, Receiver, Sender},
    helper::format_green,
};
use anyhow::Result;
use clap::ArgMatches;

pub fn send_bulk(matches: &ArgMatches) -> Result<(), anyhow::Error> {
    if matches.is_present(arg::VERBOSE) {
        println!("matches: {:#?}", matches);
    }

    let sender = Sender::init(matches)?;
    let df_receiver = Receiver::dataframe(matches)?;
    let default_message = Message::build(matches)?;
    let bulk_email = BulkEmail::build(matches, sender, &df_receiver, &default_message)?;

    if matches.is_present(arg::DISPLAY) {
        println!("Display emails: {:#?}", bulk_email);
    }

    if matches.is_present(arg::DRY_RUN) {
        println!("Dry run: {}", format_green("activated"));
    }

    if matches.is_present(arg::ASSUME_YES) {
        bulk_email.process(matches)?;
    } else {
        let confirmation = bulk_email.confirm()?;
        match confirmation {
            Confirmed::Yes => {
                bulk_email.process(matches)?;
            }
            Confirmed::No => (),
        }
    }

    Ok(())
}
