use crate::{arg, email_builder::MessageTemplate};
use clap::ArgMatches;

pub fn init(matches: &ArgMatches) -> Result<(), anyhow::Error> {
    if matches.is_present(arg::VERBOSE) {
        println!("matches: {:#?}", matches);
    }

    MessageTemplate::create(matches)
}
