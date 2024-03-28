use crate::{
    arg::{self},
    email_builder::{BulkEmail, Confirmed, Message, Receiver, Sender},
    helper::format_green,
};
use anyhow::anyhow;
use anyhow::Result;
use clap::ArgMatches;
use std::path::Path;

pub fn send_bulk(matches: &ArgMatches) -> Result<(), anyhow::Error> {
    if matches.is_present(arg::VERBOSE) {
        println!("matches: {:#?}", matches);
    }

    let sender = Sender::init(matches)?;
    let receiver_column_name = Receiver::column_name(matches)?;
    let df_receiver = Receiver::dataframe(matches)?;
    let default_message = Message::from_args(matches)?;
    let attachment = matches.value_of(arg::ATTACHMENT).map(Path::new);
    let bulk_email = if matches.is_present(arg::PERSONALIZE) {
        if let Some(personalized_columns) = matches.values_of(arg::PERSONALIZE) {
            BulkEmail::personalize(
                &sender,
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
            &sender,
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
