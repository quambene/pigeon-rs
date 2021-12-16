use crate::{
    arg::{self},
    cmd,
    data_sources::{query_postgres, write_csv, write_image},
};
use anyhow::{anyhow, Result};
use clap::{Arg, ArgMatches};

pub fn query_args() -> [Arg<'static, 'static>; 6] {
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
            .help("Save query result"),
        Arg::with_name(arg::FILE_TYPE)
            .long(arg::FILE_TYPE)
            .takes_value(false)
            .default_value("csv")
            .possible_values(&["csv", "jpg", "png"]),
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
                    // If argument 'FILE_TYPE' is not present the default value 'csv' will be used
                    match matches.value_of(arg::FILE_TYPE) {
                        Some(file_type) => match file_type {
                            x if x == "csv" => write_csv(df_query_result)?,
                            x if x == "jpg" => write_image(df_query_result, x)?,
                            x if x == "png" => write_image(df_query_result, x)?,
                            _ => {
                                return Err(anyhow!(
                                    "Value '{}' not supported for argument '{}'",
                                    file_type,
                                    arg::FILE_TYPE
                                ))
                            }
                        },
                        None => {
                            return Err(anyhow!("Missing value for argument '{}'", arg::FILE_TYPE))
                        }
                    };
                }

                Ok(())
            }
            None => Err(anyhow!("Missing value for argument '{}'", cmd::QUERY)),
        }
    } else {
        Err(anyhow!("Missing argument '{}'", cmd::QUERY))
    }
}
