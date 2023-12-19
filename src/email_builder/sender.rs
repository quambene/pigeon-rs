use anyhow::anyhow;
use clap::ArgMatches;

use crate::arg;

pub struct Sender;

impl Sender {
    pub fn init<'a>(matches: &'a ArgMatches) -> Result<&'a str, anyhow::Error> {
        if matches.is_present(arg::SENDER) {
            match matches.value_of(arg::SENDER) {
                Some(sender) => Ok(sender),
                None => Err(anyhow!("Missing value for argument '{}'", arg::SENDER)),
            }
        } else {
            Err(anyhow!("Missing argument '{}'", arg::SENDER))
        }
    }
}
