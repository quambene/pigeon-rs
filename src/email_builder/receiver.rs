use crate::arg;
use anyhow::anyhow;
use clap::ArgMatches;

pub struct Receiver;

impl Receiver {
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
