use super::{MockClient, SendEmail, SentEmail, SmtpClient};
use crate::{
    arg::{self, val},
    email_builder::Email,
    email_provider::AwsSesClient,
};
use anyhow::anyhow;
use clap::ArgMatches;

pub struct Client<'a>(pub Box<dyn SendEmail<'a>>);

impl<'a> Client<'a> {
    pub fn from_args(matches: &ArgMatches) -> Result<Self, anyhow::Error> {
        if matches.is_present(arg::DRY_RUN) {
            return Ok(Client(Box::new(MockClient)));
        }

        if matches.is_present(arg::CONNECTION) {
            match matches.value_of(arg::CONNECTION) {
                Some(connection) => match connection.to_lowercase().as_str() {
                    val::SMTP => {
                        let client = SmtpClient::new()?;
                        Ok(Client(Box::new(client)))
                    }
                    val::AWS => {
                        let client = AwsSesClient::new(matches)?;
                        Ok(Client(Box::new(client)))
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

    pub fn send(&self, email: &'a Email<'a>) -> Result<SentEmail<'a>, anyhow::Error> {
        self.0.send(email)
    }
}
