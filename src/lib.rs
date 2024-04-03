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
use clap::{crate_name, crate_version, App, Arg, SubCommand};

/// Create the CLI app to get the matches.
pub fn app() -> App<'static, 'static> {
    App::new(crate_name!())
        .version(crate_version!())
        .arg(
            clap::Arg::with_name(arg::VERBOSE)
                .long(arg::VERBOSE)
                .takes_value(false)
                .help("Shows what is going on"),
        )
        .subcommand(
            SubCommand::with_name(cmd::INIT)
                .about("Create template files in current directory")
                .args(&[Arg::with_name(arg::VERBOSE)
                    .long(arg::VERBOSE)
                    .takes_value(false)
                    .help("Shows what is going on for subcommand")]),
        )
        .subcommand(
            SubCommand::with_name(cmd::CONNECT)
                .about("Check connection to SMTP server or email provider")
                .args(&[
                    Arg::with_name(cmd::CONNECT)
                        .takes_value(true)
                        .possible_values(&[val::SMTP, val::AWS])
                        .default_value(val::SMTP)
                        .help("Check connection to SMTP server."),
                    Arg::with_name(arg::VERBOSE)
                        .long(arg::VERBOSE)
                        .takes_value(false)
                        .help("Shows what is going on for subcommand"),
                ]),
        )
        .subcommand(
            SubCommand::with_name(cmd::QUERY)
                .about("Query database and display results in terminal (select statements only)")
                .args(&[
                    Arg::with_name(cmd::QUERY)
                        .index(1)
                        .required(true)
                        .takes_value(true)
                        .help("Takes a sql query"),
                    Arg::with_name(arg::SSH_TUNNEL)
                        .long(arg::SSH_TUNNEL)
                        .value_name("port")
                        .takes_value(true)
                        .help("Connect to db through ssh tunnel"),
                    Arg::with_name(arg::SAVE)
                        .long(arg::SAVE)
                        .takes_value(false)
                        .help("Save query result"),
                    Arg::with_name(arg::SAVE_DIR)
                        .long(arg::SAVE_DIR)
                        .takes_value(true)
                        .default_value("./saved_queries")
                        .help("Specifies the output directory for saved query"),
                    Arg::with_name(arg::FILE_TYPE)
                        .long(arg::FILE_TYPE)
                        .takes_value(true)
                        .default_value("csv")
                        .possible_values(&["csv", "jpg", "png"])
                        .help("Specifies the file type for saved query"),
                    Arg::with_name(arg::IMAGE_COLUMN)
                        .long(arg::IMAGE_COLUMN)
                        .required_ifs(&[(arg::FILE_TYPE, "jpg"), (arg::FILE_TYPE, "png")])
                        .takes_value(true)
                        .help("Specifies the column in which to look for images"),
                    Arg::with_name(arg::IMAGE_NAME)
                        .long(arg::IMAGE_NAME)
                        .required_ifs(&[(arg::FILE_TYPE, "jpg"), (arg::FILE_TYPE, "png")])
                        .takes_value(true)
                        .help("Specifies the column used for the image name"),
                    Arg::with_name(arg::DISPLAY)
                        .long(arg::DISPLAY)
                        .takes_value(false)
                        .help("Print query result to terminal"),
                    Arg::with_name(arg::VERBOSE)
                        .long(arg::VERBOSE)
                        .takes_value(false)
                        .help("Shows what is going on for subcommand"),
                ]),
        )
        .subcommand(
            SubCommand::with_name(cmd::SIMPLE_QUERY)
                .about("Simple query using the simple query protocol")
                .args(&[
                    Arg::with_name(cmd::SIMPLE_QUERY)
                        .index(1)
                        .required(true)
                        .takes_value(true)
                        .help("Takes a sql query"),
                    Arg::with_name(arg::VERBOSE)
                        .long(arg::VERBOSE)
                        .takes_value(false)
                        .help("Shows what is going on for subcommand"),
                ]),
        )
        .subcommand(
            SubCommand::with_name(cmd::READ)
                .about("Read csv file and display results in terminal")
                .args(&[
                    Arg::with_name(cmd::READ).required(true).takes_value(true),
                    Arg::with_name(arg::VERBOSE)
                        .long(arg::VERBOSE)
                        .takes_value(false)
                        .help("Shows what is going on for subcommand"),
                    Arg::with_name(arg::DISPLAY)
                        .long(arg::DISPLAY)
                        .takes_value(false)
                        .help("Display csv file in terminal"),
                ]),
        )
        .subcommand(
            SubCommand::with_name(cmd::SEND)
                .about("Send email to single recipient")
                .args(&[
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
                        .required_unless_one(&[
                            arg::SUBJECT,
                            arg::CONTENT,
                            arg::TEXT_FILE,
                            arg::HTML_FILE,
                        ])
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
                ]),
        )
        .subcommand(
            SubCommand::with_name(cmd::SEND_BULK)
                .about("Send email to multiple recipients")
                .args(&[
                    Arg::with_name(arg::SENDER)
                        .index(1)
                        .required(true)
                        .takes_value(true)
                        .help("Email address of the sender"),
                    Arg::with_name(arg::RECEIVER_FILE)
                        .long(arg::RECEIVER_FILE)
                        .required_unless(arg::RECEIVER_QUERY)
                        .takes_value(true)
                        .help(
                            "Email addresses of multiple receivers fetched from provided csv file",
                        ),
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
                        .required_unless_one(&[
                            arg::SUBJECT,
                            arg::CONTENT,
                            arg::TEXT_FILE,
                            arg::HTML_FILE,
                        ])
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
                ]),
        )
}
