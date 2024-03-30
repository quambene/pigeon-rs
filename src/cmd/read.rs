use crate::{arg, cmd, data_sources};
use anyhow::{anyhow, Result};
use clap::ArgMatches;
use std::path::PathBuf;

pub fn read(matches: &ArgMatches) -> Result<(), anyhow::Error> {
    if matches.is_present(arg::VERBOSE) {
        println!("matches: {:#?}", matches);
    }

    if matches.is_present(cmd::READ) {
        match matches.value_of(cmd::READ) {
            Some(csv_file) => {
                let path = PathBuf::from(csv_file);
                let csv = data_sources::read_csv(&path)?;

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
