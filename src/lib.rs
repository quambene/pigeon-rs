/*
    Pigeon is a command line tool for automating your email workflow.
*/

/// The supported arguments.
pub mod arg;
/// The supported subcommands.
pub mod cmd;
mod email_builder;
mod email_formatter;
mod email_provider;
mod email_transmission;
mod sources;
mod utils;

use arg::val;
use clap::{crate_name, crate_version, Arg, Command};

/// Create the CLI app to get the matches.
pub fn app() -> Command {
    Command::new(crate_name!())
        .version(crate_version!())
        .arg(
            Arg::new(arg::VERBOSE)
                .long(arg::VERBOSE)
                .num_args(0)
                .required(false)
                .help("Shows what is going on"),
        )
        .subcommand(
            Command::new(cmd::INIT)
                .about("Create template files in current directory")
                .args(&[verbose()]),
        )
        .subcommand(
            Command::new(cmd::CONNECT)
                .about("Check connection to SMTP server or email provider")
                .args(&[
                    Arg::new(cmd::CONNECT)
                        .required(true)
                        .value_parser([val::SMTP, val::AWS])
                        .default_value(val::SMTP)
                        .help("Check connection to SMTP server."),
                    verbose(),
                ]),
        )
        .subcommand(
            Command::new(cmd::QUERY)
                .about("Query database and display results in terminal (select statements only)")
                .args(&[
                    Arg::new(cmd::QUERY)
                        .index(1)
                        .num_args(1)
                        .required(true)
                        .help("Takes a sql query"),
                    Arg::new(arg::SSH_TUNNEL)
                        .long(arg::SSH_TUNNEL)
                        .value_name("port")
                        .num_args(1)
                        .required(false)
                        .help("Connect to db through ssh tunnel"),
                    Arg::new(arg::SAVE)
                        .long(arg::SAVE)
                        .num_args(0)
                        .required(false)
                        .help("Save query result"),
                    Arg::new(arg::SAVE_DIR)
                        .long(arg::SAVE_DIR)
                        .num_args(1)
                        .default_value("./saved_queries")
                        .help("Specifies the output directory for saved query"),
                    Arg::new(arg::FILE_TYPE)
                        .long(arg::FILE_TYPE)
                        .num_args(1)
                        .required(false)
                        .default_value("csv")
                        .value_parser(["csv", "jpg", "png"])
                        .help("Specifies the file type for saved query"),
                    Arg::new(arg::IMAGE_COLUMN)
                        .long(arg::IMAGE_COLUMN)
                        .num_args(1)
                        .required(false)
                        .required_if_eq_any([(arg::FILE_TYPE, "jpg"), (arg::FILE_TYPE, "png")])
                        .help("Specifies the column in which to look for images"),
                    Arg::new(arg::IMAGE_NAME)
                        .long(arg::IMAGE_NAME)
                        .num_args(1)
                        .required(false)
                        .required_if_eq_any([(arg::FILE_TYPE, "jpg"), (arg::FILE_TYPE, "png")])
                        .help("Specifies the column used for the image name"),
                    display().help("Print query result to terminal"),
                    verbose(),
                ]),
        )
        .subcommand(
            Command::new(cmd::SIMPLE_QUERY)
                .about("Simple query using the simple query protocol")
                .args(&[
                    Arg::new(cmd::SIMPLE_QUERY)
                        .index(1)
                        .required(true)
                        .help("Takes a sql query"),
                    verbose(),
                ]),
        )
        .subcommand(
            Command::new(cmd::READ)
                .about("Read csv file and display results in terminal")
                .args(&[
                    Arg::new(cmd::READ).num_args(1).required(true),
                    verbose(),
                    display().help("Display csv file in terminal"),
                ]),
        )
        .subcommand(
            Command::new(cmd::SEND)
                .about("Send email to single recipient")
                .args(&[
                    Arg::new(arg::SENDER)
                        .index(1)
                        .num_args(1)
                        .required(true)
                        .requires_all([arg::RECEIVER])
                        .help("Email address of the sender"),
                    Arg::new(arg::RECEIVER)
                        .index(2)
                        .num_args(1)
                        .required(true)
                        .requires_all([arg::SENDER])
                        .help("Email address of the receiver"),
                    Arg::new(arg::SUBJECT)
                        .long(arg::SUBJECT)
                        .num_args(1)
                        .required(false)
                        .required_unless_present(arg::MESSAGE_FILE)
                        .help("Subject of the email"),
                    Arg::new(arg::CONTENT)
                        .long(arg::CONTENT)
                        .num_args(1)
                        .required(false)
                        .requires(arg::SUBJECT)
                        .required_unless_present_any([
                            arg::MESSAGE_FILE,
                            arg::TEXT_FILE,
                            arg::HTML_FILE,
                        ])
                        .conflicts_with_all([arg::MESSAGE_FILE, arg::TEXT_FILE, arg::HTML_FILE])
                        .help("Content of the email"),
                    Arg::new(arg::MESSAGE_FILE)
                        .long(arg::MESSAGE_FILE)
                        .num_args(1)
                        .required(false)
                        .required_unless_present_any([
                            arg::SUBJECT,
                            arg::CONTENT,
                            arg::TEXT_FILE,
                            arg::HTML_FILE,
                        ])
                        .conflicts_with_all([arg::CONTENT, arg::TEXT_FILE, arg::HTML_FILE])
                        .help("Path of the message file"),
                    Arg::new(arg::TEXT_FILE)
                        .long(arg::TEXT_FILE)
                        .num_args(1)
                        .required(false)
                        .requires(arg::SUBJECT)
                        .conflicts_with_all([arg::CONTENT, arg::MESSAGE_FILE])
                        .help("Path of text file"),
                    Arg::new(arg::HTML_FILE)
                        .long(arg::HTML_FILE)
                        .num_args(1)
                        .required(false)
                        .requires(arg::SUBJECT)
                        .conflicts_with_all([arg::CONTENT, arg::MESSAGE_FILE])
                        .help("Path of html file"),
                    Arg::new(arg::ATTACHMENT)
                        .long(arg::ATTACHMENT)
                        .num_args(1)
                        .required(false)
                        .help("Path of attachment"),
                    archive(),
                    archive_dir(),
                    display().help("Display email in terminal"),
                    dry_run().help("Prepare email but do not send email"),
                    assume_yes().help("Send email without confirmation"),
                    Arg::new(arg::CONNECTION)
                        .long(arg::CONNECTION)
                        .num_args(1)
                        .required(false)
                        .value_parser([val::SMTP, val::AWS])
                        .default_value(val::SMTP)
                        .help("Send emails via SMTP or AWS API"),
                    verbose(),
                ]),
        )
        .subcommand(
            Command::new(cmd::SEND_BULK)
                .about("Send email to multiple recipients")
                .args(&[
                    Arg::new(arg::SENDER)
                        .index(1)
                        .num_args(1)
                        .required(true)
                        .help("Email address of the sender"),
                    Arg::new(arg::RECEIVER_FILE)
                        .long(arg::RECEIVER_FILE)
                        .num_args(1)
                        .required(false)
                        .required_unless_present(arg::RECEIVER_QUERY)
                        .help(
                            "Email addresses of multiple receivers fetched from provided csv file",
                        ),
                    Arg::new(arg::RECEIVER_QUERY)
                        .long(arg::RECEIVER_QUERY)
                        .num_args(1)
                        .required(false)
                        .required_unless_present(arg::RECEIVER_FILE)
                        .help("Email addresses of multiple receivers fetched from provided query"),
                    Arg::new(arg::SUBJECT)
                        .long(arg::SUBJECT)
                        .num_args(1)
                        .required(false)
                        .required_unless_present(arg::MESSAGE_FILE)
                        .help("Subject of the email"),
                    Arg::new(arg::CONTENT)
                        .long(arg::CONTENT)
                        .num_args(1)
                        .required(false)
                        .requires(arg::SUBJECT)
                        .required_unless_present_any([
                            arg::MESSAGE_FILE,
                            arg::TEXT_FILE,
                            arg::HTML_FILE,
                        ])
                        .conflicts_with_all([arg::MESSAGE_FILE, arg::TEXT_FILE, arg::HTML_FILE])
                        .help("Content of the email"),
                    Arg::new(arg::MESSAGE_FILE)
                        .long(arg::MESSAGE_FILE)
                        .num_args(1)
                        .required(false)
                        .required_unless_present_any([
                            arg::SUBJECT,
                            arg::CONTENT,
                            arg::TEXT_FILE,
                            arg::HTML_FILE,
                        ])
                        .conflicts_with_all([arg::CONTENT, arg::TEXT_FILE, arg::HTML_FILE])
                        .help("Path of the message file"),
                    Arg::new(arg::TEXT_FILE)
                        .long(arg::TEXT_FILE)
                        .num_args(1)
                        .required(false)
                        .requires(arg::SUBJECT)
                        .conflicts_with_all([arg::CONTENT, arg::MESSAGE_FILE])
                        .help("Path of text file"),
                    Arg::new(arg::HTML_FILE)
                        .long(arg::HTML_FILE)
                        .num_args(1)
                        .required(false)
                        .requires(arg::SUBJECT)
                        .conflicts_with_all([arg::CONTENT, arg::MESSAGE_FILE])
                        .help("Path of html file"),
                    Arg::new(arg::ATTACHMENT)
                        .long(arg::ATTACHMENT)
                        .num_args(1)
                        .required(false)
                        .help("Path of attachment"),
                    archive(),
                    archive_dir(),
                    Arg::new(arg::RECEIVER_COLUMN)
                        .long(arg::RECEIVER_COLUMN)
                        .num_args(1)
                        .required(false)
                        .default_value(val::EMAIL)
                        .help("Specifies the column in which to look for email addresses"),
                    Arg::new(arg::PERSONALIZE)
                        .long(arg::PERSONALIZE)
                        .num_args(0..100)
                        .required(false)
                        .help("Personalizes email for variables defined in the message template"),
                    display().help("Print emails to terminal"),
                    dry_run().help("Prepare emails but do not send emails"),
                    assume_yes().help("Send emails without confirmation"),
                    Arg::new(arg::SSH_TUNNEL)
                        .long(arg::SSH_TUNNEL)
                        .value_name("port")
                        .num_args(1)
                        .required(false)
                        .help("Query db through ssh tunnel"),
                    Arg::new(arg::CONNECTION)
                        .long(arg::CONNECTION)
                        .num_args(1)
                        .required(false)
                        .value_parser([val::SMTP, val::AWS])
                        .default_value(val::SMTP)
                        .help("Send emails via SMTP or AWS API"),
                    verbose(),
                ]),
        )
}

fn verbose() -> Arg {
    Arg::new(arg::VERBOSE)
        .long(arg::VERBOSE)
        .num_args(0)
        .required(false)
        .help("Shows what is going on for subcommand")
}

fn display() -> Arg {
    Arg::new(arg::DISPLAY)
        .long(arg::DISPLAY)
        .num_args(0)
        .required(false)
}

fn dry_run() -> Arg {
    Arg::new(arg::DRY_RUN)
        .long(arg::DRY_RUN)
        .num_args(0)
        .required(false)
}

fn assume_yes() -> Arg {
    Arg::new(arg::ASSUME_YES)
        .long(arg::ASSUME_YES)
        .num_args(0)
        .required(false)
}

fn archive() -> Arg {
    Arg::new(arg::ARCHIVE)
        .long(arg::ARCHIVE)
        .num_args(0)
        .required(false)
        .help("Archive sent emails")
}

fn archive_dir() -> Arg {
    Arg::new(arg::ARCHIVE_DIR)
        .long(arg::ARCHIVE_DIR)
        .num_args(1)
        .required(false)
        .default_value("./sent_emails")
        .help("Path of sent emails")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_send_args_subject_content() {
        let args = vec![
            "pigeon",
            "send",
            "albert@einstein.com",
            "marie@curie.com",
            "--subject",
            "Test subject",
            "--content",
            "This is a test message (plaintext).",
        ];
        let app = app();
        let matches = app.get_matches_from(args);
        let subcommand_matches = matches.subcommand_matches(cmd::SEND);
        dbg!(&subcommand_matches);
        assert!(subcommand_matches.is_some());

        let subcommand_matches = subcommand_matches.unwrap();
        assert!(subcommand_matches.contains_id(arg::SUBJECT));
        assert!(subcommand_matches.contains_id(arg::CONTENT));
        assert!(!subcommand_matches.contains_id(arg::ATTACHMENT));
        assert!(!subcommand_matches.get_flag(arg::VERBOSE));
        assert!(!subcommand_matches.get_flag(arg::DRY_RUN));
        assert!(!subcommand_matches.get_flag(arg::DISPLAY));
        assert!(!subcommand_matches.get_flag(arg::ASSUME_YES));
        assert!(!subcommand_matches.get_flag(arg::ARCHIVE));
        assert!(subcommand_matches.contains_id(arg::ARCHIVE_DIR));
        assert_eq!(
            subcommand_matches
                .get_one::<String>(arg::ARCHIVE_DIR)
                .unwrap(),
            "./sent_emails"
        );
    }

    #[test]
    fn test_send_args_text_file_html_file() {
        let args = vec![
            "pigeon",
            "send",
            "albert@einstein.com",
            "marie@curie.com",
            "--subject",
            "Test subject",
            "--text-file",
            "./test_data/message.txt",
            "--html-file",
            "./test_data/message.html",
        ];
        let app = app();
        let matches = app.get_matches_from(args);
        let subcommand_matches = matches.subcommand_matches(cmd::SEND);
        assert!(subcommand_matches.is_some());

        let subcommand_matches = subcommand_matches.unwrap();
        assert!(subcommand_matches.contains_id(arg::SUBJECT));
        assert!(subcommand_matches.contains_id(arg::TEXT_FILE));
        assert!(subcommand_matches.contains_id(arg::HTML_FILE));
        assert!(!subcommand_matches.contains_id(arg::ATTACHMENT));
        assert!(!subcommand_matches.get_flag(arg::VERBOSE));
        assert!(!subcommand_matches.get_flag(arg::DRY_RUN));
        assert!(!subcommand_matches.get_flag(arg::DISPLAY));
        assert!(!subcommand_matches.get_flag(arg::ASSUME_YES));
        assert!(!subcommand_matches.get_flag(arg::ARCHIVE));
        assert!(subcommand_matches.contains_id(arg::ARCHIVE_DIR));
        assert_eq!(
            subcommand_matches
                .get_one::<String>(arg::ARCHIVE_DIR)
                .unwrap(),
            "./sent_emails"
        );
    }

    #[test]
    fn test_send_args_message_file() {
        let args = vec![
            "pigeon",
            "send",
            "albert@einstein.com",
            "marie@curie.com",
            "--message-file",
            "./test_data/message.yaml",
        ];
        let app = app();
        let matches = app.get_matches_from(args);
        let subcommand_matches = matches.subcommand_matches(cmd::SEND);
        assert!(subcommand_matches.is_some());

        let subcommand_matches = subcommand_matches.unwrap();
        assert!(subcommand_matches.contains_id(arg::MESSAGE_FILE));
        assert!(!subcommand_matches.contains_id(arg::ATTACHMENT));
        assert!(!subcommand_matches.get_flag(arg::VERBOSE));
        assert!(!subcommand_matches.get_flag(arg::DRY_RUN));
        assert!(!subcommand_matches.get_flag(arg::DISPLAY));
        assert!(!subcommand_matches.get_flag(arg::ASSUME_YES));
        assert!(!subcommand_matches.get_flag(arg::ARCHIVE));
        assert!(subcommand_matches.contains_id(arg::ARCHIVE_DIR));
        assert_eq!(
            subcommand_matches
                .get_one::<String>(arg::ARCHIVE_DIR)
                .unwrap(),
            "./sent_emails"
        );
    }
}
