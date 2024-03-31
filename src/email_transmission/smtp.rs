use super::{SendEmail, SentEmail, Status};
use crate::{
    email_builder::Email,
    utils::{format_green, format_red},
};
use anyhow::Context;
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
                "Connecting to SMTP server '{}' ... {}",
                endpoint,
                format_red("FAILED")
            ))?
            .credentials(credentials)
            .build();

        println!(
            "Connecting to SMTP server '{}' ... {}",
            endpoint,
            format_green("ok")
        );

        Ok(Self {
            endpoint,
            transport,
        })
    }
}

impl<'a> SendEmail<'a> for SmtpClient {
    fn send(&self, email: &'a Email<'a>) -> Result<SentEmail<'a>, anyhow::Error> {
        let response = self
            .transport
            .send(&email.mime_format.message)
            .context("Can't sent email via SMTP");
        let status = match response {
            Ok(response) => {
                let response_string = response.message().collect::<String>();
                let messages: Vec<&str> = response_string.split(' ').collect();
                let message_id = messages[1];
                Status::SentOk(message_id.to_string())
            }
            Err(err) => Status::SentError(err.to_string()),
        };
        let sent_email = SentEmail::new(email, status);

        Ok(sent_email)
    }
}
