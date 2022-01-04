use super::{client::SendEmail, SentEmail, Status};
use crate::{
    arg,
    email_builder::Email,
    helper::{format_green, format_red},
};
use anyhow::Context;
use clap::ArgMatches;
use lettre::{transport::smtp::authentication::Credentials, SmtpTransport, Transport};
use std::env;

pub struct SmtpClient {
    pub endpoint: String,
    pub transport: SmtpTransport,
}

impl SmtpClient {
    // TLS connection to SMTP server
    pub fn new() -> Result<Self, anyhow::Error> {
        let endpoint =
            env::var("SMTP_SERVER").context("Missing environment variable 'SMTP_SERVER'")?;
        let username =
            env::var("SMTP_USERNAME").context("Missing environment variable 'SMTP_USERNAME'")?;
        let password =
            env::var("SMTP_PASSWORD").context("Missing environment variable 'SMTP_PASSWORD'")?;
        let credentials = Credentials::new(username, password);

        let transport = SmtpTransport::relay(endpoint.as_str())
            .context(format!(
                "Connecting to smtp server '{}' ... {}",
                endpoint,
                format_red("FAILED")
            ))?
            .credentials(credentials)
            .build();

        println!(
            "Connecting to smtp server '{}' ... {}",
            endpoint,
            format_green("ok")
        );

        Ok(Self {
            endpoint,
            transport,
        })
    }

    pub fn display_connection_status(&self, connection: &str) {
        println!(
            "Connected to {} server '{}' ... {}",
            connection,
            self.endpoint,
            format_green("ok")
        );
    }
}

impl<'a> SendEmail<'a> for SmtpClient {
    fn send(
        &self,
        matches: &ArgMatches,
        email: &'a Email<'a>,
    ) -> Result<SentEmail<'a>, anyhow::Error> {
        let sent_email = if matches.is_present(arg::DRY_RUN) {
            let status = Status::DryRun;
            SentEmail::new(email, status)
        } else {
            let response = self
                .transport
                .send(&email.mime_format.message)
                .context("Can't sent email via SMTP");
            let status = match response {
                Ok(response) => Status::SentOk(response.message().collect()),
                Err(err) => Status::SentError(err.to_string()),
            };
            SentEmail::new(email, status)
        };

        Ok(sent_email)
    }
}
