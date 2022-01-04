use crate::{
    arg::{self, val},
    cmd,
    email_provider::AwsSesClient,
    email_transmission::SmtpClient,
};
use anyhow::{anyhow, Result};
use clap::{Arg, ArgMatches};

pub fn connect_args() -> [Arg<'static, 'static>; 2] {
    [
        Arg::with_name(cmd::CONNECT)
            .takes_value(true)
            .possible_values(&[val::SMTP, val::AWS])
            .default_value(val::SMTP)
            .help("Check connection to SMTP server."),
        Arg::with_name(arg::VERBOSE)
            .long(arg::VERBOSE)
            .takes_value(false)
            .help("Shows what is going on for subcommand"),
    ]
}

pub fn connect(matches: &ArgMatches) -> Result<(), anyhow::Error> {
    if matches.is_present(arg::VERBOSE) {
        println!("matches: {:#?}", matches);
    }

    if matches.is_present(cmd::CONNECT) {
        match matches.value_of(cmd::CONNECT) {
            Some(connection) => match connection {
                val::SMTP => {
                    let _client = SmtpClient::new()?;
                    Ok(())
                }
                val::AWS => {
                    let _client = AwsSesClient::new(matches)?;
                    Ok(())
                }
                other => Err(anyhow!(
                    "Value '{}' for argument '{}' not supported",
                    other,
                    cmd::CONNECT
                )),
            },
            None => Err(anyhow!("Missing value for argument '{}'", cmd::CONNECT)),
        }
    } else {
        Err(anyhow!("Missing argument '{}'", cmd::CONNECT))
    }
}
