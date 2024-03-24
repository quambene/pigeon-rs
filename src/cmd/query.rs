use crate::{
    arg::{self},
    cmd,
    data_sources::{query_postgres, write_csv, write_image},
};
use anyhow::{anyhow, Result};
use clap::ArgMatches;

pub fn query(matches: &ArgMatches) -> Result<(), anyhow::Error> {
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
                            "csv" => write_csv(matches, df_query_result)?,
                            x if x == "jpg" => write_image(matches, df_query_result, x)?,
                            x if x == "png" => write_image(matches, df_query_result, x)?,
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
