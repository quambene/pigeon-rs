use crate::{arg, cmd, data_sources::read_csv};
use anyhow::{anyhow, Result};
use clap::{Arg, ArgMatches};
use std::path::PathBuf;

pub fn read_args() -> [Arg<'static, 'static>; 3] {
    [
        Arg::with_name(cmd::READ).required(true).takes_value(true),
        Arg::with_name(arg::VERBOSE)
            .long(arg::VERBOSE)
            .takes_value(false)
            .help("Shows what is going on for subcommand"),
        Arg::with_name(arg::DISPLAY)
            .long(arg::DISPLAY)
            .takes_value(false)
            .help("Display csv file in terminal"),
    ]
}

pub fn read(matches: &ArgMatches) -> Result<(), anyhow::Error> {
    if matches.is_present(arg::VERBOSE) {
        println!("matches: {:#?}", matches);
    }

    if matches.is_present(cmd::READ) {
        match matches.value_of(cmd::READ) {
            Some(csv_file) => {
                let path = PathBuf::from(csv_file);
                let csv = read_csv(&path)?;

                if matches.is_present(arg::DISPLAY) {
                    println!("Display csv file: {}", csv);
                }

                Ok(())
            }
            None => Err(anyhow!("Missing value for argument '{}'", cmd::READ)),
        }
    } else {
        Err(anyhow!("Missing argument '{}'", cmd::READ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{app, cmd};

    #[test]
    fn test_read() {
        let args = vec![cmd::BIN, cmd::READ, "./test_data/receiver.csv"];

        let app = app();
        let matches = app.get_matches_from(args);
        let subcommand_matches = matches.subcommand_matches(cmd::READ).unwrap();
        println!("subcommand matches: {:#?}", subcommand_matches);

        let res = read(&subcommand_matches);
        println!("res: {:#?}", res);

        assert!(res.is_ok())
    }
}
