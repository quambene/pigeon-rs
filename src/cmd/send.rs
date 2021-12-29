use crate::{
    arg,
    email_builder::{Confirmed, Email},
    email_transmission::Client,
    helper::format_green,
};
use anyhow::Result;
use clap::{Arg, ArgMatches};

pub fn send_args() -> [Arg<'static, 'static>; 12] {
    [
        Arg::with_name(arg::SENDER)
            .index(1)
            .required(true)
            .takes_value(true)
            .requires_all(&[arg::RECEIVER])
            .help("Email address of the sender"),
        Arg::with_name(arg::RECEIVER)
            .index(2)
            .required(true)
            .takes_value(true)
            .requires_all(&[arg::SENDER])
            .help("Email address of the receiver"),
        Arg::with_name(arg::SUBJECT)
            .long(arg::SUBJECT)
            .takes_value(true)
            .requires(arg::CONTENT)
            .required_unless_one(&[arg::MESSAGE_FILE])
            .help("Subject of the email"),
        Arg::with_name(arg::CONTENT)
            .long(arg::CONTENT)
            .takes_value(true)
            .requires(arg::SUBJECT)
            .required_unless_one(&[arg::MESSAGE_FILE])
            .help("Content of the email"),
        Arg::with_name(arg::MESSAGE_FILE)
            .long(arg::MESSAGE_FILE)
            .takes_value(true)
            .required_unless_one(&[arg::SUBJECT, arg::CONTENT])
            .help("Path of the message file"),
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
        Arg::with_name(arg::DISPLAY)
            .long(arg::DISPLAY)
            .takes_value(false)
            .help("Display email in terminal"),
        Arg::with_name(arg::DRY_RUN)
            .long(arg::DRY_RUN)
            .takes_value(false)
            .help("Prepare email but do not send email"),
        Arg::with_name(arg::ASSUME_YES)
            .long(arg::ASSUME_YES)
            .takes_value(false)
            .help("Send email without confirmation"),
        Arg::with_name(arg::VERBOSE)
            .long(arg::VERBOSE)
            .takes_value(false)
            .help("Shows what is going on for subcommand"),
    ]
}

pub fn send(matches: &ArgMatches<'_>) -> Result<(), anyhow::Error> {
    if matches.is_present(arg::VERBOSE) {
        println!("matches: {:#?}", matches);
    }

    let email = Email::new(matches)?;

    if matches.is_present(arg::DISPLAY) {
        println!("Display email: {:#?}", email);
    }

    if matches.is_present(arg::DRY_RUN) {
        println!("Dry run: {}", format_green("activated"));
    }

    let client = Client::new()?;

    println!("Sending email to 1 recipient ...");

    if matches.is_present(arg::ASSUME_YES) {
        client.send(matches, &email)?;
        println!(
            "{:#?} ... {:#?}",
            email.receiver,
            email.status.try_borrow()?
        );
        email.archive(matches)?;
    } else {
        let confirmation = email.confirm(matches)?;
        match confirmation {
            Confirmed::Yes => {
                client.send(matches, &email)?;
                println!(
                    "{:#?} ... {:#?}",
                    email.receiver,
                    email.status.try_borrow()?
                );
                email.archive(matches)?;
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{app, cmd};
    use std::env;

    #[test]
    fn test_send_subject_content_dry() {
        let args = vec![
            cmd::BIN,
            cmd::SEND,
            "albert@einstein.com",
            "marie@curie.com",
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
        let subcommand_matches = matches.subcommand_matches(cmd::SEND).unwrap();
        println!("subcommand matches: {:#?}", subcommand_matches);

        let res = send(&subcommand_matches);
        println!("res: {:#?}", res);

        assert!(res.is_ok())
    }

    #[test]
    #[ignore]
    fn test_send_subject_content() {
        let sender = env::var("TEST_SENDER").expect("Missing environment variable 'TEST_SENDER'");
        let receiver =
            env::var("TEST_RECEIVER").expect("Missing environment variable 'TEST_RECEIVER'");

        let args = vec![
            cmd::BIN,
            cmd::SEND,
            &sender,
            &receiver,
            "--subject",
            "Test Subject",
            "--content",
            "This is a test message (plaintext).",
            "--display",
            "--assume-yes",
        ];

        let app = app();
        let matches = app.get_matches_from(args);
        let subcommand_matches = matches.subcommand_matches(cmd::SEND).unwrap();
        println!("subcommand matches: {:#?}", subcommand_matches);

        let res = send(&subcommand_matches);
        println!("res: {:#?}", res);

        assert!(res.is_ok())
    }

    #[test]
    fn test_send_message_file_dry() {
        let args = vec![
            cmd::BIN,
            cmd::SEND,
            "albert@einstein.com",
            "marie@curie.com",
            "--message-file",
            "./test_data/message.yaml",
            "--dry-run",
            "--display",
            "--assume-yes",
        ];

        let app = app();
        let matches = app.get_matches_from(args);
        let subcommand_matches = matches.subcommand_matches(cmd::SEND).unwrap();
        println!("subcommand matches: {:#?}", subcommand_matches);

        let res = send(&subcommand_matches);
        println!("res: {:#?}", res);

        assert!(res.is_ok())
    }

    #[test]
    fn test_archive_dry() {
        let args = vec![
            cmd::BIN,
            cmd::SEND,
            "albert@einstein.com",
            "marie@curie.com",
            "--message-file",
            "./test_data/message.yaml",
            "--dry-run",
            "--display",
            "--assume-yes",
            "--archive",
        ];

        let app = app();
        let matches = app.get_matches_from(args);
        let subcommand_matches = matches.subcommand_matches(cmd::SEND).unwrap();
        println!("subcommand matches: {:#?}", subcommand_matches);

        let res = send(&subcommand_matches);
        println!("res: {:#?}", res);

        assert!(res.is_ok())
    }

    #[test]
    fn test_archive_dir_dry() {
        let args = vec![
            cmd::BIN,
            cmd::SEND,
            "albert@einstein.com",
            "marie@curie.com",
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
        let subcommand_matches = matches.subcommand_matches(cmd::SEND).unwrap();
        println!("subcommand matches: {:#?}", subcommand_matches);

        let res = send(&subcommand_matches);
        println!("res: {:#?}", res);

        assert!(res.is_ok())
    }

    #[test]
    fn test_attachment_pdf_dry() {
        let args = vec![
            cmd::BIN,
            cmd::SEND,
            "albert@einstein.com",
            "marie@curie.com",
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
        let subcommand_matches = matches.subcommand_matches(cmd::SEND).unwrap();
        println!("subcommand matches: {:#?}", subcommand_matches);

        let res = send(&subcommand_matches);
        println!("res: {:#?}", res);

        assert!(res.is_ok())
    }

    #[test]
    fn test_attachment_png_dry() {
        let args = vec![
            cmd::BIN,
            cmd::SEND,
            "albert@einstein.com",
            "marie@curie.com",
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
        let subcommand_matches = matches.subcommand_matches(cmd::SEND).unwrap();
        println!("subcommand matches: {:#?}", subcommand_matches);

        let res = send(&subcommand_matches);
        println!("res: {:#?}", res);

        assert!(res.is_ok())
    }

    #[test]
    fn test_attachment_odt_dry() {
        let args = vec![
            cmd::BIN,
            cmd::SEND,
            "albert@einstein.com",
            "marie@curie.com",
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
        let subcommand_matches = matches.subcommand_matches(cmd::SEND).unwrap();
        println!("subcommand matches: {:#?}", subcommand_matches);

        let res = send(&subcommand_matches);
        println!("res: {:#?}", res);

        assert!(res.is_ok())
    }

    #[test]
    #[ignore]
    fn test_send_message_file() {
        let sender = env::var("TEST_SENDER").expect("Missing environment variable 'TEST_SENDER'");
        let receiver =
            env::var("TEST_RECEIVER").expect("Missing environment variable 'TEST_RECEIVER'");

        let args = vec![
            cmd::BIN,
            cmd::SEND,
            &sender,
            &receiver,
            "--message-file",
            "./test_data/message.yaml",
            "--display",
            "--assume-yes",
        ];

        let app = app();
        let matches = app.get_matches_from(args);
        let subcommand_matches = matches.subcommand_matches(cmd::SEND).unwrap();
        println!("subcommand matches: {:#?}", subcommand_matches);

        let res = send(&subcommand_matches);
        println!("res: {:#?}", res);

        assert!(res.is_ok())
    }

    #[test]
    #[ignore]
    fn test_attachment_pdf() {
        let sender = env::var("TEST_SENDER").expect("Missing environment variable 'TEST_SENDER'");
        let receiver =
            env::var("TEST_RECEIVER").expect("Missing environment variable 'TEST_RECEIVER'");

        let args = vec![
            cmd::BIN,
            cmd::SEND,
            &sender,
            &receiver,
            "--message-file",
            "./test_data/message.yaml",
            "--display",
            "--assume-yes",
            "--archive",
            "--attachment",
            "./test_data/test.pdf",
        ];

        let app = app();
        let matches = app.get_matches_from(args);
        let subcommand_matches = matches.subcommand_matches(cmd::SEND).unwrap();
        println!("subcommand matches: {:#?}", subcommand_matches);

        let res = send(&subcommand_matches);
        println!("res: {:#?}", res);

        assert!(res.is_ok())
    }
}
