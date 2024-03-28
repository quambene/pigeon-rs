use crate::{arg, cmd, data_loader::TabularData};
use anyhow::anyhow;
use clap::ArgMatches;
use polars::prelude::DataFrame;

#[derive(Debug, Clone, Copy)]
pub struct Receiver<'a>(pub &'a str);

impl<'a> Receiver<'a> {
    pub fn as_str(&self) -> &str {
        self.0
    }

    pub fn from_args(matches: &'a ArgMatches) -> Result<Self, anyhow::Error> {
        if matches.is_present(arg::RECEIVER) {
            match matches.value_of(arg::RECEIVER) {
                Some(receiver) => Ok(Self(receiver)),
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

    pub fn column_name(matches: &'a ArgMatches<'a>) -> Result<&str, anyhow::Error> {
        // If argument 'RECEIVER_COLUMN' is not present the default value 'email' will be used as column name
        match matches.value_of(arg::RECEIVER_COLUMN) {
            Some(column_name) => Ok(column_name),
            None => Err(anyhow!(
                "Missing value for argument '{}'",
                arg::RECEIVER_COLUMN
            )),
        }
    }

    pub fn file_name(matches: &'a ArgMatches<'a>) -> Result<&str, anyhow::Error> {
        match matches.value_of(arg::RECEIVER_FILE) {
            Some(file_name) => Ok(file_name),
            None => Err(anyhow!(
                "Missing value for argument '{}'",
                arg::RECEIVER_FILE
            )),
        }
    }

    pub fn query(matches: &'a ArgMatches<'a>) -> Result<&str, anyhow::Error> {
        match matches.value_of(arg::RECEIVER_QUERY) {
            Some(query) => Ok(query),
            None => Err(anyhow!(
                "Missing value for argument '{}'",
                arg::RECEIVER_QUERY
            )),
        }
    }
}
