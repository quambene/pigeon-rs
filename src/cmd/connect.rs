use crate::{arg, cmd, email_provider::setup_ses_client, helper::format_green};
use anyhow::{anyhow, Result};
use clap::{Arg, ArgMatches};

pub fn connect_args() -> [Arg<'static, 'static>; 2] {
    [
        Arg::with_name(cmd::CONNECT)
            .takes_value(true)
            .possible_values(&["aws"]),
        Arg::with_name(arg::VERBOSE)
            .long(arg::VERBOSE)
            .takes_value(false)
            .help("Shows what is going on for subcommand"),
    ]
}

pub fn connect(matches: &ArgMatches<'_>) -> Result<(), anyhow::Error> {
    if matches.is_present(arg::VERBOSE) {
        println!("matches: {:#?}", matches);
    }

    if matches.is_present(cmd::CONNECT) {
        match matches.value_of(cmd::CONNECT) {
            Some(provider) => {
                let client_res = setup_ses_client(matches);

                match client_res {
                    Ok(_) => {
                        println!("Connected to {} client: {}", provider, format_green("ok"));
                        Ok(())
                    }
                    Err(err) => Err(anyhow!(
                        "Can't establish connection to {}: {:#?}",
                        provider,
                        err
                    )),
                }
            }
            None => Err(anyhow!("Missing value for argument '{}'", cmd::CONNECT)),
        }
    } else {
        Err(anyhow!("Missing argument '{}'", cmd::CONNECT))
    }
}
