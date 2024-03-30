use crate::arg;
use anyhow::anyhow;
use clap::ArgMatches;

#[derive(Debug, Clone, Copy, PartialEq)]
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
}
