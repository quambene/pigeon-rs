use crate::arg;
use anyhow::anyhow;
use clap::ArgMatches;

#[derive(Debug)]
pub struct Sender<'a>(pub &'a str);

impl<'a> Sender<'a> {
    pub fn init(matches: &'a ArgMatches) -> Result<Sender<'a>, anyhow::Error> {
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
