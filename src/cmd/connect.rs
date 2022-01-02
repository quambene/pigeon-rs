use crate::{
    arg, cmd, email_provider::AwsSesClient, email_transmission::SmtpClient, helper::format_green,
};
use anyhow::{anyhow, Result};
use clap::{Arg, ArgMatches};

pub fn connect_args() -> [Arg<'static, 'static>; 2] {
    [
        Arg::with_name(cmd::CONNECT)
            .takes_value(true)
            .possible_values(&["smtp", "aws"])
            .default_value("smtp")
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
            Some(provider) => match provider {
                x if x == "smtp" => {
                    let client = SmtpClient::new();

                    println!("Connecting to {} server ...", x);

                    match client {
                        Ok(client) => {
                            println!(
                                "Connected to {} server '{}' ... {}",
                                x,
                                client.endpoint,
                                format_green("ok")
                            );
                            Ok(())
                        }
                        Err(err) => Err(anyhow!(
                            "Can't establish connection to {} server: {:#?}",
                            x,
                            err
                        )),
                    }
                }
                x if x == "aws" => {
                    let client = AwsSesClient::new(matches);

                    println!("Connecting to {} server ...", x);

                    match client {
                        Ok(client) => {
                            println!(
                                "Connected to {} server with region '{}' ... {}",
                                x,
                                client.region_name,
                                format_green("ok")
                            );
                            Ok(())
                        }
                        Err(err) => Err(anyhow!(
                            "Can't establish connection to {} server: {:#?}",
                            x,
                            err
                        )),
                    }
                }
                _ => Err(anyhow!("Invalid value for argument '{}'", cmd::CONNECT)),
            },
            None => Err(anyhow!("Missing value for argument '{}'", cmd::CONNECT)),
        }
    } else {
        Err(anyhow!("Missing argument '{}'", cmd::CONNECT))
    }
}
