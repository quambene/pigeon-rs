use crate::{
    arg::{self, val},
    email_builder::{BulkEmail, Confirmed, Message, Receiver, Sender},
    helper::format_green,
};
use anyhow::Result;
use clap::{Arg, ArgMatches};

pub fn send_bulk_args() -> [Arg<'static, 'static>; 19] {
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
            .help("Email addresses of multiple receivers fetched from provided query"),
        Arg::with_name(arg::SUBJECT)
            .long(arg::SUBJECT)
            .takes_value(true)
            .required_unless_one(&[arg::MESSAGE_FILE])
            .help("Subject of the email"),
        Arg::with_name(arg::CONTENT)
            .long(arg::CONTENT)
            .takes_value(true)
            .requires(arg::SUBJECT)
            .required_unless_one(&[arg::MESSAGE_FILE, arg::TEXT_FILE, arg::HTML_FILE])
            .conflicts_with_all(&[arg::MESSAGE_FILE, arg::TEXT_FILE, arg::HTML_FILE])
            .help("Content of the email"),
        Arg::with_name(arg::MESSAGE_FILE)
            .long(arg::MESSAGE_FILE)
            .takes_value(true)
            .required_unless_one(&[arg::SUBJECT, arg::CONTENT, arg::TEXT_FILE, arg::HTML_FILE])
            .conflicts_with_all(&[arg::CONTENT, arg::TEXT_FILE, arg::HTML_FILE])
            .help("Path of the message file"),
        Arg::with_name(arg::TEXT_FILE)
            .long(arg::TEXT_FILE)
            .takes_value(true)
            .requires(arg::SUBJECT)
            .conflicts_with_all(&[arg::CONTENT, arg::MESSAGE_FILE])
            .help("Path of text file"),
        Arg::with_name(arg::HTML_FILE)
            .long(arg::HTML_FILE)
            .takes_value(true)
            .requires(arg::SUBJECT)
            .conflicts_with_all(&[arg::CONTENT, arg::MESSAGE_FILE])
            .help("Path of html file"),
        Arg::with_name(arg::ATTACHMENT)
            .long(arg::ATTACHMENT)
            .takes_value(true)
            .help("Path of attachment"),
        Arg::with_name(arg::ARCHIVE)
            .long(arg::ARCHIVE)
            .takes_value(false)
            .help("Archive sent emails"),
        Arg::with_name(arg::ARCHIVE_DIR)
            .long(arg::ARCHIVE_DIR)
            .takes_value(true)
            .default_value("./sent_emails")
            .help("Path of sent emails"),
        Arg::with_name(arg::RECEIVER_COLUMN)
            .long(arg::RECEIVER_COLUMN)
            .takes_value(true)
            .default_value(val::EMAIL)
            .help("Specifies the column in which to look for email addresses"),
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
        Arg::with_name(arg::SSH_TUNNEL)
            .long(arg::SSH_TUNNEL)
            .value_name("port")
            .takes_value(true)
            .help("Query db through ssh tunnel"),
        Arg::with_name(arg::CONNECTION)
            .long(arg::CONNECTION)
            .takes_value(true)
            .possible_values(&[val::SMTP, val::AWS])
            .default_value(val::SMTP)
            .help("Send emails via SMTP or AWS API"),
        Arg::with_name(arg::VERBOSE)
            .long(arg::VERBOSE)
            .takes_value(false)
            .help("Shows what is going on for subcommand"),
    ]
}

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{app, cmd};

    #[test]
    fn test_send_bulk_subject_content_dry() {
        let args = vec![
            cmd::BIN,
            cmd::SEND_BULK,
            "albert@einstein.com",
            "--receiver-file",
            "./test_data/receiver.csv",
            "--subject",
            "Test Subject",
            "--content",
            "This is a test message (plaintext).",
            "--dry-run",
            "--display",
            "--assume-yes",
        ];

        let app = app();
        let matches = app.get_matches_from(args);
        let subcommand_matches = matches.subcommand_matches(cmd::SEND_BULK).unwrap();
        println!("subcommand matches: {:#?}", subcommand_matches);

        let res = send_bulk(subcommand_matches);
        println!("res: {:#?}", res);

        assert!(res.is_ok())
    }

    #[test]
    fn test_send_bulk_text_file_html_file_dry() {
        let args = vec![
            cmd::BIN,
            cmd::SEND_BULK,
            "albert@einstein.com",
            "--receiver-file",
            "./test_data/receiver.csv",
            "--subject",
            "Test Subject",
            "--text-file",
            "./test_data/message.txt",
            "--html-file",
            "./test_data/message.html",
            "--dry-run",
            "--display",
            "--assume-yes",
        ];

        let app = app();
        let matches = app.get_matches_from(args);
        let subcommand_matches = matches.subcommand_matches(cmd::SEND_BULK).unwrap();
        println!("subcommand matches: {:#?}", subcommand_matches);

        let res = send_bulk(subcommand_matches);
        println!("res: {:#?}", res);

        assert!(res.is_ok())
    }

    #[test]
    fn test_send_bulk_message_file_dry() {
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

        let res = send_bulk(subcommand_matches);
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

        let res = send_bulk(subcommand_matches);
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
            "first_name",
            "last_name",
            "--dry-run",
            "--display",
            "--assume-yes",
        ];

        let app = app();
        let matches = app.get_matches_from(args);
        let subcommand_matches = matches.subcommand_matches(cmd::SEND_BULK).unwrap();
        println!("subcommand matches: {:#?}", subcommand_matches);

        let res = send_bulk(subcommand_matches);
        println!("res: {:#?}", res);

        assert!(res.is_ok())
    }

    #[test]
    fn test_archive_dry() {
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
            "--archive",
        ];

        let app = app();
        let matches = app.get_matches_from(args);
        let subcommand_matches = matches.subcommand_matches(cmd::SEND_BULK).unwrap();
        println!("subcommand matches: {:#?}", subcommand_matches);

        let res = send_bulk(subcommand_matches);
        println!("res: {:#?}", res);

        assert!(res.is_ok())
    }

    #[test]
    fn test_archive_dir_dry() {
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
            "--archive",
            "--archive-dir",
            "./my-sent-emails",
        ];

        let app = app();
        let matches = app.get_matches_from(args);
        let subcommand_matches = matches.subcommand_matches(cmd::SEND_BULK).unwrap();
        println!("subcommand matches: {:#?}", subcommand_matches);

        let res = send_bulk(subcommand_matches);
        println!("res: {:#?}", res);

        assert!(res.is_ok())
    }

    #[test]
    fn test_attachment_pdf_dry() {
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
            "--archive",
            "--attachment",
            "./test_data/test.pdf",
        ];

        let app = app();
        let matches = app.get_matches_from(args);
        let subcommand_matches = matches.subcommand_matches(cmd::SEND_BULK).unwrap();
        println!("subcommand matches: {:#?}", subcommand_matches);

        let res = send_bulk(subcommand_matches);
        println!("res: {:#?}", res);

        assert!(res.is_ok())
    }

    #[test]
    fn test_attachment_png_dry() {
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
            "--archive",
            "--attachment",
            "./test_data/test.png",
        ];

        let app = app();
        let matches = app.get_matches_from(args);
        let subcommand_matches = matches.subcommand_matches(cmd::SEND_BULK).unwrap();
        println!("subcommand matches: {:#?}", subcommand_matches);

        let res = send_bulk(subcommand_matches);
        println!("res: {:#?}", res);

        assert!(res.is_ok())
    }

    #[test]
    fn test_attachment_odt_dry() {
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
            "--archive",
            "--attachment",
            "./test_data/test.odt",
        ];

        let app = app();
        let matches = app.get_matches_from(args);
        let subcommand_matches = matches.subcommand_matches(cmd::SEND_BULK).unwrap();
        println!("subcommand matches: {:#?}", subcommand_matches);

        let res = send_bulk(subcommand_matches);
        println!("res: {:#?}", res);

        assert!(res.is_ok())
    }

    #[test]
    fn test_send_bulk_aws_dry() {
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
            "--connection",
            val::AWS,
        ];

        let app = app();
        let matches = app.get_matches_from(args);
        let subcommand_matches = matches.subcommand_matches(cmd::SEND_BULK).unwrap();
        println!("subcommand matches: {:#?}", subcommand_matches);

        let res = send_bulk(subcommand_matches);
        println!("res: {:#?}", res);

        assert!(res.is_ok())
    }
}
