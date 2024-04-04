use crate::{arg, cmd, sources};
use anyhow::{anyhow, Result};
use clap::ArgMatches;
use std::path::PathBuf;

pub fn read(matches: &ArgMatches) -> Result<(), anyhow::Error> {
    if matches.contains_id(arg::VERBOSE) {
        println!("matches: {:#?}", matches);
    }

    if matches.contains_id(cmd::READ) {
        match matches.get_one::<String>(cmd::READ) {
            Some(csv_file) => {
                let path = PathBuf::from(csv_file);
                let csv = sources::read_csv(&path)?;

                if matches.contains_id(arg::DISPLAY) {
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
