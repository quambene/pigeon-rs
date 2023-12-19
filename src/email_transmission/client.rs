use super::{SentEmail, SmtpClient};
use crate::{
    arg::{self, val},
    email_builder::Email,
    email_provider::AwsSesClient,
};
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
    pub fn init<'a>(matches: &ArgMatches) -> Result<Box<dyn SendEmail<'a>>, anyhow::Error> {
        if matches.is_present(arg::CONNECTION) {
            match matches.value_of(arg::CONNECTION) {
                Some(connection) => match connection.to_lowercase().as_str() {
                    val::SMTP => {
                        let client = SmtpClient::new()?;
                        Ok(Box::new(client))
                    }
                    val::AWS => {
                        let client = AwsSesClient::new(matches)?;
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
