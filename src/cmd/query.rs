use crate::{
    arg::{self},
    cmd,
    data_sources::{query_postgres, write_csv, write_image},
};
use anyhow::{anyhow, Result};
use clap::{Arg, ArgMatches};

pub fn query_args() -> [Arg<'static, 'static>; 9] {
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
    ]
}

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{app, cmd};
    use std::env;

    #[test]
    #[ignore]
    fn test_display_query() {
        let test_query = env::var("TEST_QUERY").expect("Missing environment variable 'TEST_QUERY'");
        let args = vec![cmd::BIN, cmd::QUERY, test_query.as_str(), "--display"];

        let app = app();
        let matches = app.get_matches_from(args);
        let subcommand_matches = matches.subcommand_matches(cmd::QUERY).unwrap();
        println!("subcommand matches: {:#?}", subcommand_matches);

        let res = query(subcommand_matches);
        println!("res: {:#?}", res);

        assert!(res.is_ok())
    }

    #[test]
    #[ignore]
    fn test_save_query() {
        let test_query = env::var("TEST_QUERY").expect("Missing environment variable 'TEST_QUERY'");
        let args = vec![
            cmd::BIN,
            cmd::QUERY,
            test_query.as_str(),
            "--display",
            "--save",
        ];

        let app = app();
        let matches = app.get_matches_from(args);
        let subcommand_matches = matches.subcommand_matches(cmd::QUERY).unwrap();
        println!("subcommand matches: {:#?}", subcommand_matches);

        let res = query(subcommand_matches);
        println!("res: {:#?}", res);

        assert!(res.is_ok())
    }

    #[test]
    #[ignore]
    fn test_save_dir() {
        let test_query = env::var("TEST_QUERY").expect("Missing environment variable 'TEST_QUERY'");
        let args = vec![
            cmd::BIN,
            cmd::QUERY,
            test_query.as_str(),
            "--display",
            "--save",
            "--save-dir",
            "./my-saved-queries",
        ];

        let app = app();
        let matches = app.get_matches_from(args);
        let subcommand_matches = matches.subcommand_matches(cmd::QUERY).unwrap();
        println!("subcommand matches: {:#?}", subcommand_matches);

        let res = query(subcommand_matches);
        println!("res: {:#?}", res);

        assert!(res.is_ok())
    }
}
