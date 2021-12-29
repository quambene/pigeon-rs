use super::SentEmail;
use crate::{
    arg,
    email_builder::{Email, Status},
};
use anyhow::Context;
use clap::ArgMatches;
use lettre::{transport::smtp::authentication::Credentials, SmtpTransport, Transport};
use std::env;

pub struct Client {
    transport: SmtpTransport,
}

impl Client {
    // TLS connection to SMTP server
    pub fn new() -> Result<Self, anyhow::Error> {
        let endpoint = env::var("SMTP_SERVER").expect("Missing environment variable 'SMTP_SERVER'");
        let username =
            env::var("SMTP_USERNAME").expect("Missing environment variable 'SMTP_USERNAME'");
        let password =
            env::var("SMTP_PASSWORD").expect("Missing environment variable 'SMTP_PASSWORD'");
        let credentials = Credentials::new(username, password);
        let transport = SmtpTransport::relay(endpoint.as_str())
            .context("Can't connect to smtp server")?
            .credentials(credentials)
            .build();
        Ok(Self { transport })
    }

    pub fn send(
        &self,
        matches: &ArgMatches<'_>,
        email: &Email,
    ) -> Result<SentEmail, anyhow::Error> {
        let sent_email = if matches.is_present(arg::DRY_RUN) {
            let status = Status::DryRun;
            SentEmail::new(email, status)
        } else {
            let response = self
                .transport
                .send(&email.mime_format.message)
                .context("Can't sent email via SMTP");
            let status = match response {
                Ok(response) => Status::Sent(response.message().collect()),
                Err(err) => Status::SentError(err.to_string()),
            };
            SentEmail::new(email, status)
        };

        Ok(sent_email)
    }
}
