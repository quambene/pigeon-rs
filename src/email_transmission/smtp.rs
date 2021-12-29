use crate::email_builder::Email;
use anyhow::Context;
use lettre::{transport::smtp::authentication::Credentials, SmtpTransport, Transport};
use std::env;

pub struct Mailer {
    transport: SmtpTransport,
}

impl Mailer {
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

    pub fn send(&self, email: &Email) -> Result<String, anyhow::Error> {
        let response = self
            .transport
            .send(&email.mime.message)
            .context("Can't sent email via SMTP")?;
        let message_ids = response.message().collect();
        Ok(message_ids)
    }
}
