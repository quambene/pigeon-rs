use crate::arg;
use anyhow::{anyhow, Context};
use clap::ArgMatches;
use lettre::{
    message::{header, MultiPart, SinglePart},
    FileTransport, Message, Transport,
};
use std::{path::PathBuf, str};

#[derive(Debug)]
pub struct Mime {
    pub message: Message,
}

impl Mime {
    pub fn new(
        sender: &str,
        receiver: &str,
        message: &super::Message,
    ) -> Result<Self, anyhow::Error> {
        let text: &str = match &message.text {
            Some(text) => text,
            None => Default::default(),
        };
        let html: &str = match &message.html {
            Some(text) => text,
            None => Default::default(),
        };
        let message = Message::builder()
            .from(sender.parse().context("Can't parse sender")?)
            .to(receiver.parse().context("Can't parse receiver")?)
            .subject(&message.subject)
            .multipart(
                MultiPart::alternative()
                    .singlepart(
                        SinglePart::builder()
                            .header(header::ContentType::TEXT_PLAIN)
                            .body(text.to_string()),
                    )
                    .singlepart(
                        SinglePart::builder()
                            .header(header::ContentType::TEXT_HTML)
                            .body(html.to_string()),
                    ),
            )
            .context("Can't create email")?;

        Ok(Self { message })
    }

    pub fn archive(&self, matches: &ArgMatches<'_>) -> Result<(), anyhow::Error> {
        let target_dir = match matches.value_of(arg::ARCHIVE_DIR) {
            Some(archive_dir) => PathBuf::from(archive_dir),
            None => return Err(anyhow!("Missing value for argument '{}'", arg::ARCHIVE_DIR)),
        };
        let mailer = FileTransport::new(target_dir);
        mailer
            .send(&self.message)
            .context("Can't save email in .eml format")?;

        Ok(())
    }

    pub fn display(&self) -> Result<(), anyhow::Error> {
        println!(
            "Display MIME formatted email:\n{}",
            str::from_utf8(&self.message.formatted())?
        );

        Ok(())
    }
}
