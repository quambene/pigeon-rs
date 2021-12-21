use crate::{arg, email_builder};
use anyhow::{anyhow, Context};
use clap::ArgMatches;
use lettre::{
    message::{header, MultiPart, SinglePart},
    FileTransport, Message, Transport,
};
use std::{fmt, path::PathBuf, str};

pub struct Mime {
    pub message: Message,
}

impl Mime {
    pub fn new(
        sender: &str,
        receiver: &str,
        message: &email_builder::Message,
    ) -> Result<Self, anyhow::Error> {
        let message_builder = Message::builder()
            .from(sender.parse().context("Can't parse sender")?)
            .to(receiver.parse().context("Can't parse receiver")?)
            .subject(&message.subject);

        let message = match (&message.text, &message.html) {
            (Some(text), Some(html)) => message_builder.multipart(Mime::alternative(text, html)),
            (Some(text), None) => message_builder.singlepart(Mime::text_plain(text)),
            (None, Some(html)) => message_builder.singlepart(Mime::text_html(html)),
            (_, _) => return Err(anyhow!("Missing email body")),
        }
        .context("Can't create MIME formatted email")?;

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

    fn text_plain(text: &str) -> SinglePart {
        SinglePart::builder()
            .header(header::ContentType::TEXT_PLAIN)
            .body(text.to_string())
    }

    fn text_html(text: &str) -> SinglePart {
        SinglePart::builder()
            .header(header::ContentType::TEXT_HTML)
            .body(text.to_string())
    }

    fn attachment(file_type: &str, file_name: &str) -> Result<SinglePart, anyhow::Error> {
        Ok(SinglePart::builder()
            .header(header::ContentType::parse("application/pdf")?)
            .header(header::ContentDisposition::attachment(file_name))
            .body(Vec::<u8>::default()))
    }

    fn alternative(text: &str, html: &str) -> MultiPart {
        MultiPart::alternative()
            .singlepart(Mime::text_plain(text))
            .singlepart(Mime::text_plain(html))
    }

    fn mixed(
        text: &str,
        html: &str,
        file_type: &str,
        file_name: &str,
    ) -> Result<MultiPart, anyhow::Error> {
        Ok(MultiPart::mixed()
            .singlepart(Mime::text_plain(text))
            .singlepart(Mime::text_plain(html))
            .singlepart(Mime::attachment(file_type, file_name)?))
    }
}

impl fmt::Debug for Mime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            str::from_utf8(&self.message.formatted()).expect("Can't convert from utf8")
        )
    }
}
