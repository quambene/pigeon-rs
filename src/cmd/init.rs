use crate::{arg, email_builder::MessageTemplate};
use clap::{Arg, ArgMatches};

pub fn init_args() -> [Arg<'static, 'static>; 1] {
    [Arg::with_name(arg::VERBOSE)
        .long(arg::VERBOSE)
        .takes_value(false)
        .help("Shows what is going on for subcommand")]
}

pub fn init(matches: &ArgMatches<'_>) -> Result<(), anyhow::Error> {
    if matches.is_present(arg::VERBOSE) {
        println!("matches: {:#?}", matches);
    }

    MessageTemplate::create(matches)
}
