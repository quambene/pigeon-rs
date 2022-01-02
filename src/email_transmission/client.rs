use super::{SentEmail, SmtpClient};
use crate::{arg, email_builder::Email, email_provider::AwsSesClient, helper::format_green};
use anyhow::anyhow;
use clap::ArgMatches;

pub trait SendEmail<'a> {
    fn send(
        &self,
        matches: &ArgMatches,
        email: &'a Email<'a>,
    ) -> Result<SentEmail<'a>, anyhow::Error>;
}

pub struct Client;

impl Client {
    pub fn new<'a>(matches: &ArgMatches) -> Result<Box<dyn SendEmail<'a>>, anyhow::Error> {
        println!("Connecting to email server ...");

        if matches.is_present(arg::CONNECTION) {
            match matches.value_of(arg::CONNECTION) {
                Some(connection) => match connection {
                    "smtp" => {
                        let client = SmtpClient::new()?;
                        println!(
                            "Connected to {} server '{}' ... {}",
                            connection,
                            client.endpoint,
                            format_green("ok")
                        );
                        Ok(Box::new(client))
                    }
                    "aws" => {
                        let client = AwsSesClient::new(matches)?;
                        println!(
                            "Connected to {} server with region '{}' ... {}",
                            connection,
                            client.region_name,
                            format_green("ok")
                        );
                        Ok(Box::new(client))
                    }
                    other => Err(anyhow!(format!(
                        "Value '{}' for argument '{}' not supported",
                        other,
                        arg::CONNECTION
                    ))),
                },
                None => Err(anyhow!(format!(
                    "Missing value for argument '{}'",
                    arg::CONNECTION
                ))),
            }
        } else {
            Err(anyhow!(format!("Missing argument '{}'", arg::CONNECTION)))
        }
    }
}
