use super::{MockClient, SendEmail, SentEmail, SmtpClient};
use crate::{
    arg::{self, val},
    email_builder::Email,
    email_provider::AwsSesClient,
};
use anyhow::anyhow;
use clap::ArgMatches;
use std::fmt;

#[derive(Debug, PartialEq)]
pub enum TransmissionType {
    Smtp,
    Aws,
    Dry,
}

impl fmt::Display for TransmissionType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let request_method = match self {
            Self::Smtp => "smtp",
            Self::Aws => "aws",
            Self::Dry => "dry",
        };

        write!(f, "{}", request_method)
    }
}

pub struct Client<'a> {
    #[allow(dead_code)]
    transmission_type: TransmissionType,
    client: Box<dyn SendEmail<'a>>,
}

impl<'a> Client<'a> {
    pub fn new(transmission_type: TransmissionType, client: Box<dyn SendEmail<'a>>) -> Self {
        Self {
            transmission_type,
            client,
        }
    }

    pub fn from_args(matches: &ArgMatches) -> Result<Self, anyhow::Error> {
        if matches.is_present(arg::DRY_RUN) {
            let client = MockClient;
            return Ok(Client::new(TransmissionType::Dry, Box::new(client)));
        }

        let connection = arg::value(arg::CONNECTION, matches)?;

        match connection.to_lowercase().as_str() {
            val::SMTP => {
                let client = SmtpClient::new()?;
                Ok(Client::new(TransmissionType::Smtp, Box::new(client)))
            }
            val::AWS => {
                let client = AwsSesClient::new(matches)?;
                Ok(Client::new(TransmissionType::Aws, Box::new(client)))
            }
            other => Err(anyhow!(format!(
                "Value '{}' for argument '{}' not supported",
                other,
                arg::CONNECTION
            ))),
        }
    }

    pub fn send(&self, email: &'a Email<'a>) -> Result<SentEmail<'a>, anyhow::Error> {
        self.client.send(email)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::app;

    #[test]
    fn test_client_from_args_dry() {
        let args = vec![
            "pigeon",
            "send",
            "albert@einstein.com",
            "marie@curie.com",
            "--message-file",
            "./test_data/message.yaml",
            "--dry-run",
        ];
        let app = app();
        let matches = app.get_matches_from(args);
        let subcommand_matches = matches.subcommand_matches("send").unwrap();

        let res = Client::from_args(subcommand_matches);
        assert!(res.is_ok());

        let client = res.unwrap();
        assert_eq!(client.transmission_type, TransmissionType::Dry);
    }
}
