#![feature(generic_associated_types)]

mod arg;
mod cmd;
mod data_sources;
mod email_handler;
mod email_provider;
mod helper;

use anyhow::anyhow;
use clap::{crate_name, crate_version, App, SubCommand};
use std::env;

fn main() -> Result<(), anyhow::Error> {
    let app = app();
    let matches = app.get_matches();

    if matches.is_present(arg::VERBOSE) {
        println!("matches: {:#?}", matches);
    }

    match matches.subcommand() {
        (cmd::INIT, Some(matches)) => cmd::init(matches),
        (cmd::CONNECT, Some(matches)) => cmd::connect(matches),
        (cmd::QUERY, Some(matches)) => cmd::query(matches),
        (cmd::SIMPLE_QUERY, Some(matches)) => cmd::simple_query(matches),
        (cmd::READ, Some(matches)) => cmd::read(matches),
        (cmd::SEND, Some(matches)) => cmd::send(matches),
        (cmd::SEND_BULK, Some(matches)) => cmd::send_bulk(matches),
        (_, _) => Err(anyhow!("Subcommand not found")),
    }
}

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
                .about("Check connection to the provider specified")
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
                .about("Send single email via command line")
                .args(&cmd::send_args()),
        )
        .subcommand(
            SubCommand::with_name(cmd::SEND_BULK)
                .about("Send email to multiple recipients")
                .args(&cmd::send_bulk_args()),
        )
}
