use crate::{
    arg, cmd,
    sources::{self, ConnVars, DbConnection},
};
use anyhow::{anyhow, Result};
use chrono::Utc;
use clap::ArgMatches;
use std::path::Path;

pub fn query(matches: &ArgMatches) -> Result<(), anyhow::Error> {
    if matches.is_present(arg::VERBOSE) {
        println!("matches: {:#?}", matches);
    }

    if matches.is_present(cmd::QUERY) {
        match matches.value_of(cmd::QUERY) {
            Some(query) => {
                let now = Utc::now();
                let conn_vars = ConnVars::from_env()?;
                let ssh_tunnel = matches.value_of(arg::SSH_TUNNEL);

                let connection = DbConnection::new(&conn_vars, ssh_tunnel)?;
                let mut df_query = sources::query_postgres(&connection, query)?;

                if matches.is_present(arg::DISPLAY) {
                    println!("Display query result: {}", df_query);
                }

                if matches.is_present(arg::SAVE) {
                    let save_dir = Path::new(arg::value(arg::SAVE_DIR, matches)?);

                    // If argument 'FILE_TYPE' is not present the default value 'csv' will be used
                    match matches.value_of(arg::FILE_TYPE) {
                        Some(file_type) => match file_type {
                            "csv" => {
                                sources::write_csv(&mut df_query, save_dir, now)?;
                            }
                            "jpg" | "png" => {
                                let image_column = arg::value(arg::IMAGE_COLUMN, matches)?;
                                let image_name = arg::value(arg::IMAGE_NAME, matches)?;
                                sources::write_image(
                                    save_dir,
                                    image_column,
                                    image_name,
                                    df_query,
                                    file_type,
                                )?;
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
