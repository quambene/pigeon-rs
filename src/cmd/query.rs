use crate::{
    arg::{self},
    cmd,
    data_sources::{query_postgres, write_csv},
};
use anyhow::{anyhow, Result};
use clap::{Arg, ArgMatches};

pub fn query_args() -> [Arg<'static, 'static>; 5] {
    [
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
            .help("Save query result as csv file"),
        Arg::with_name(arg::DISPLAY)
            .long(arg::DISPLAY)
            .takes_value(false)
            .help("Print query result to terminal"),
        Arg::with_name(arg::VERBOSE)
            .long(arg::VERBOSE)
            .takes_value(false)
            .help("Shows what is going on for subcommand"),
    ]
}

pub fn query(matches: &ArgMatches<'_>) -> Result<(), anyhow::Error> {
    if matches.is_present(arg::VERBOSE) {
        println!("matches: {:#?}", matches);
    }

    if matches.is_present(cmd::QUERY) {
        match matches.value_of(cmd::QUERY) {
            Some(query) => {
                let df_query_result = query_postgres(matches, query)?;

                if matches.is_present(arg::DISPLAY) {
                    println!("Display query result: {}", df_query_result);
                }

                if matches.is_present(arg::SAVE) {
                    write_csv(df_query_result)?;
                }

                Ok(())
            }
            None => Err(anyhow!("Missing value for argument '{}'", cmd::QUERY)),
        }
    } else {
        Err(anyhow!("Missing argument '{}'", cmd::QUERY))
    }
}
