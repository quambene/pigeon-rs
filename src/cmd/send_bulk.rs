use crate::{arg, email_handler::BulkEmail, helper::format_green};
use anyhow::Result;
use clap::{Arg, ArgMatches};

pub fn send_bulk_args() -> [Arg<'static, 'static>; 10] {
    [
        Arg::with_name(arg::SENDER)
            .index(1)
            .required(true)
            .takes_value(true)
            .help("Email address of the sender"),
        Arg::with_name(arg::RECEIVER_FILE)
            .long(arg::RECEIVER_FILE)
            .required_unless(arg::RECEIVER_QUERY)
            .takes_value(true)
            .help("Email addresses of multiple receivers fetched from provided csv file"),
        Arg::with_name(arg::RECEIVER_QUERY)
            .long(arg::RECEIVER_QUERY)
            .required_unless(arg::RECEIVER_FILE)
            .takes_value(true)
            .help("Email addresses of multiple receivers fetched from provided  query"),
        Arg::with_name(arg::MESSAGE_FILE)
            .long(arg::MESSAGE_FILE)
            .required(true)
            .takes_value(true)
            .help("Path of the message file"),
        Arg::with_name(arg::RECEIVER_COLUMN)
            .long(arg::RECEIVER_COLUMN)
            .takes_value(false)
            .default_value("email")
            .help("Defines the column name in which to look for email addresses"),
        Arg::with_name(arg::PERSONALIZE)
            .long(arg::PERSONALIZE)
            .takes_value(true)
            .multiple(true)
            .max_values(100)
            .help("Personalizes email for variables defined in the message template"),
        Arg::with_name(arg::DISPLAY)
            .long(arg::DISPLAY)
            .takes_value(false)
            .help("Print emails to terminal"),
        Arg::with_name(arg::DRY_RUN)
            .long(arg::DRY_RUN)
            .takes_value(false)
            .help("Prepare emails but do not send emails"),
        Arg::with_name(arg::ASSUME_YES)
            .long(arg::ASSUME_YES)
            .takes_value(false)
            .help("Send emails without confirmation"),
        Arg::with_name(arg::VERBOSE)
            .long(arg::VERBOSE)
            .takes_value(false)
            .help("Shows what is going on for subcommand"),
    ]
}

pub fn send_bulk(matches: &ArgMatches<'_>) -> Result<(), anyhow::Error> {
    if matches.is_present(arg::VERBOSE) {
        println!("matches: {:#?}", matches);
    }

    let bulk_email = BulkEmail::new(matches)?;

    if matches.is_present(arg::DISPLAY) {
        println!("Display emails: {:#?}", bulk_email);
    }

    if matches.is_present(arg::DRY_RUN) {
        println!("Dry run: {}", format_green("activated"));
    }

    if matches.is_present(arg::ASSUME_YES) {
        bulk_email.send(matches)?;
    } else {
        bulk_email.confirm_and_send(matches)?;
    }

    bulk_email.archive(matches)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{app, cmd};

    #[test]
    fn test_send_bulk_dry() {
        let args = vec![
            cmd::BIN,
            cmd::SEND_BULK,
            "albert@einstein.com",
            "--receiver-file",
            "./test_data/receiver.csv",
            "--message-file",
            "./test_data/message.yaml",
            "--dry-run",
            "--display",
            "--assume-yes",
        ];

        let app = app();
        let matches = app.get_matches_from(args);
        let subcommand_matches = matches.subcommand_matches(cmd::SEND_BULK).unwrap();
        println!("subcommand matches: {:#?}", subcommand_matches);

        let res = send_bulk(&subcommand_matches);
        println!("res: {:#?}", res);

        assert!(res.is_ok())
    }

    #[test]
    fn test_send_bulk_receiver_column_dry() {
        let args = vec![
            cmd::BIN,
            cmd::SEND_BULK,
            "albert@einstein.com",
            "--receiver-file",
            "./test_data/contacts.csv",
            "--message-file",
            "./test_data/message.yaml",
            "--receiver-column",
            "contact",
            "--dry-run",
            "--display",
            "--assume-yes",
        ];

        let app = app();
        let matches = app.get_matches_from(args);
        let subcommand_matches = matches.subcommand_matches(cmd::SEND_BULK).unwrap();
        println!("subcommand matches: {:#?}", subcommand_matches);

        let res = send_bulk(&subcommand_matches);
        println!("res: {:#?}", res);

        assert!(res.is_ok())
    }

    #[test]
    fn test_send_bulk_personalize_dry() {
        let args = vec![
            cmd::BIN,
            cmd::SEND_BULK,
            "albert@einstein.com",
            "--receiver-file",
            "./test_data/receiver.csv",
            "--message-file",
            "./test_data/message_personalized.yaml",
            "--personalize",
            "firstname",
            "lastname",
            "--dry-run",
            "--display",
            "--assume-yes",
        ];

        let app = app();
        let matches = app.get_matches_from(args);
        let subcommand_matches = matches.subcommand_matches(cmd::SEND_BULK).unwrap();
        println!("subcommand matches: {:#?}", subcommand_matches);

        let res = send_bulk(&subcommand_matches);
        println!("res: {:#?}", res);

        assert!(res.is_ok())
    }
}
