pub mod arg;
pub mod cmd;
mod data_loader;
mod data_sources;
mod email_builder;
mod email_formatter;
mod email_provider;
mod email_transmission;
mod helper;

use clap::{crate_name, crate_version, App, SubCommand};

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
                .args(&cmd::init_args()),
        )
        .subcommand(
            SubCommand::with_name(cmd::CONNECT)
                .about("Check connection to SMTP server or email provider")
                .args(&cmd::connect_args()),
        )
        .subcommand(
            SubCommand::with_name(cmd::QUERY)
                .about("Query database and display results in terminal (select statements only)")
                .args(&cmd::query_args()),
        )
        .subcommand(
            SubCommand::with_name(cmd::SIMPLE_QUERY)
                .about("Simple query using the simple query protocol")
                .args(&cmd::simple_query_args()),
        )
        .subcommand(
            SubCommand::with_name(cmd::READ)
                .about("Read csv file and display results in terminal")
                .args(&cmd::read_args()),
        )
        .subcommand(
            SubCommand::with_name(cmd::SEND)
                .about("Send email to single recipient")
                .args(&cmd::send_args()),
        )
        .subcommand(
            SubCommand::with_name(cmd::SEND_BULK)
                .about("Send email to multiple recipients")
                .args(&cmd::send_bulk_args()),
        )
}
