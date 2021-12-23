use crate::{arg, email_builder};
use anyhow::{anyhow, Context};
use clap::ArgMatches;
use lettre::{
    message::{header, MultiPart, SinglePart},
    FileTransport, Message, Transport,
};
use std::{
    fmt, fs,
    path::{Path, PathBuf},
    str,
};

pub struct Mime {
    pub message: Message,
}

impl Mime {
    pub fn new(
        matches: &ArgMatches<'_>,
        sender: &str,
        receiver: &str,
        message: &email_builder::Message,
    ) -> Result<Self, anyhow::Error> {
        let message_builder = Message::builder()
            .from(sender.parse().context("Can't parse sender")?)
            .to(receiver.parse().context("Can't parse receiver")?)
            .subject(&message.subject);

        let message = match (
            &message.text,
            &message.html,
            matches.value_of(arg::ATTACHMENT),
        ) {
            (Some(text), Some(html), Some(attachment)) => message_builder.multipart(
                MultiPart::mixed()
                    .multipart(Mime::alternative(text, html))
                    .singlepart(Mime::attachment(attachment)?),
            ),
            (Some(text), Some(html), None) => {
                message_builder.multipart(Mime::alternative(text, html))
            }
            (Some(text), None, Some(attachment)) => message_builder.multipart(
                MultiPart::mixed()
                    .singlepart(Mime::text_plain(text))
                    .singlepart(Mime::attachment(attachment)?),
            ),
            (None, Some(html), Some(attachment)) => message_builder.multipart(
                MultiPart::mixed()
                    .singlepart(Mime::text_html(html))
                    .singlepart(Mime::attachment(attachment)?),
            ),
            (Some(text), None, None) => message_builder.singlepart(Mime::text_plain(text)),
            (None, Some(html), None) => message_builder.singlepart(Mime::text_html(html)),
            (None, None, Some(attachment)) => {
                message_builder.singlepart(Mime::attachment(attachment)?)
            }
            (None, None, None) => return Err(anyhow!("Missing email body")),
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

    fn attachment(file: &str) -> Result<SinglePart, anyhow::Error> {
        let path = Path::new(file);
        let file_name = match path.file_name() {
            Some(file_name) => match file_name.to_str() {
                Some(file_name) => file_name,
                None => {
                    return Err(anyhow!(
                        "Email attachment error: Invalid characters in file name"
                    ))
                }
            },
            None => return Err(anyhow!("Can't find attachment")),
        };
        let bytes = fs::read(path).context("Can't read attachment")?;
        let content_type = match infer::get(&bytes) {
            Some(file_type) => file_type.mime_type(),
            // Handle 'None': Compare internet standard RFC-2046, RFC-7231, and https://stackoverflow.com/questions/1176022/unknown-file-type-mime
            None => "application/octet-stream",
        };

        Ok(SinglePart::builder()
            .header(header::ContentType::parse(content_type).context(format!(
                "File type '{}' not supported: {}",
                content_type, file_name
            ))?)
            .header(header::ContentDisposition::attachment(file_name))
            .body(bytes))
    }

    fn alternative(text: &str, html: &str) -> MultiPart {
        MultiPart::alternative()
            .singlepart(Mime::text_plain(text))
            .singlepart(Mime::text_html(html))
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
