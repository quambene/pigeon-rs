use crate::{arg, cmd, data_loader::TabularData};
use anyhow::anyhow;
use clap::ArgMatches;
use polars::prelude::DataFrame;

pub struct Receiver;

impl Receiver {
    pub fn init<'a>(matches: &'a ArgMatches) -> Result<&'a str, anyhow::Error> {
        if matches.is_present(arg::RECEIVER) {
            match matches.value_of(arg::RECEIVER) {
                Some(receiver) => Ok(receiver),
                None => Err(anyhow!("Missing value for argument '{}'", arg::RECEIVER)),
            }
        } else {
            Err(anyhow!("Missing argument '{}'", arg::RECEIVER))
        }
    }

    pub fn dataframe(matches: &ArgMatches) -> Result<DataFrame, anyhow::Error> {
        match (
            matches.is_present(arg::RECEIVER_QUERY),
            matches.is_present(arg::RECEIVER_FILE),
        ) {
            (true, false) => TabularData::from_query(matches),
            (false, true) => TabularData::from_file(matches),
            (true, true) => Err(anyhow!(
                    "Argument conflict: arguments {} and {} are not allowed at the same time. Check usage via '{} help {}'",
                    arg::RECEIVER_QUERY,
                    arg::RECEIVER_FILE,
                    cmd::BIN,
                    cmd::SEND_BULK,
                )),
            (false, false) => Err(anyhow!(
                    "Missing arguments: please specify argument {} or {}. Check usage via '{} help {}'",
                    arg::RECEIVER_QUERY,
                    arg::RECEIVER_FILE,
                    cmd::BIN,
                    cmd::SEND_BULK,
                )),
        }
    }

    pub fn column_name<'a>(matches: &'a ArgMatches<'a>) -> Result<&str, anyhow::Error> {
        // If argument 'RECEIVER_COLUMN' is not present the default value 'email' will be used as column name
        match matches.value_of(arg::RECEIVER_COLUMN) {
            Some(column_name) => Ok(column_name),
            None => Err(anyhow!(
                "Missing value for argument '{}'",
                arg::RECEIVER_COLUMN
            )),
        }
    }

    pub fn file_name<'a>(matches: &'a ArgMatches<'a>) -> Result<&str, anyhow::Error> {
        match matches.value_of(arg::RECEIVER_FILE) {
            Some(file_name) => Ok(file_name),
            None => Err(anyhow!(
                "Missing value for argument '{}'",
                arg::RECEIVER_FILE
            )),
        }
    }

    pub fn query<'a>(matches: &'a ArgMatches<'a>) -> Result<&str, anyhow::Error> {
        match matches.value_of(arg::RECEIVER_QUERY) {
            Some(query) => Ok(query),
            None => Err(anyhow!(
                "Missing value for argument '{}'",
                arg::RECEIVER_QUERY
            )),
        }
    }
}
