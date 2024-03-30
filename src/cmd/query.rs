use std::path::Path;

use crate::{
    arg, cmd,
    data_sources::{self, ConnVars, DbConnection},
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
                let conn_vars = ConnVars::from_env()?;
                let ssh_tunnel = matches.value_of(arg::SSH_TUNNEL);

                let connection = DbConnection::new(&conn_vars, ssh_tunnel)?;
                let df_query_result = data_sources::query_postgres(&connection, query)?;

                if matches.is_present(arg::DISPLAY) {
                    println!("Display query result: {}", df_query_result);
                }

                if matches.is_present(arg::SAVE) {
                    // If argument 'FILE_TYPE' is not present the default value 'csv' will be used
                    match matches.value_of(arg::FILE_TYPE) {
                        Some(file_type) => match file_type {
                            "csv" => {
                                let save_dir = Path::new(arg::value(arg::SAVE_DIR, matches)?);
                                data_sources::write_csv(save_dir, df_query_result)?;
                            }
                            x if x == "jpg" => {
                                data_sources::write_image(matches, df_query_result, x)?
                            }
                            x if x == "png" => {
                                data_sources::write_image(matches, df_query_result, x)?
                            }
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
