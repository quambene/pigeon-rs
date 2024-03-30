use crate::arg;
use anyhow::anyhow;
use clap::ArgMatches;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Sender<'a>(pub &'a str);

impl<'a> Sender<'a> {
    pub fn from_args(matches: &'a ArgMatches) -> Result<Self, anyhow::Error> {
        if matches.is_present(arg::SENDER) {
            match matches.value_of(arg::SENDER) {
                Some(sender) => Ok(Sender(sender)),
                None => Err(anyhow!("Missing value for argument '{}'", arg::SENDER)),
            }
        } else {
            Err(anyhow!("Missing argument '{}'", arg::SENDER))
        }
    }
}
