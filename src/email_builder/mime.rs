use crate::{arg, email_builder};
use anyhow::{anyhow, Context};
use clap::ArgMatches;
use lettre::{
    message::{header, MultiPart, SinglePart},
    Message,
};
use std::{fmt, fs, path::Path, str};

#[derive(Clone)]
pub struct MimeFormat {
    pub message: Message,
}

impl MimeFormat {
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
                    .multipart(Self::alternative(text, html))
                    .singlepart(Self::attachment(attachment)?),
            ),
            (Some(text), Some(html), None) => {
                message_builder.multipart(Self::alternative(text, html))
            }
            (Some(text), None, Some(attachment)) => message_builder.multipart(
                MultiPart::mixed()
                    .singlepart(Self::text_plain(text))
                    .singlepart(Self::attachment(attachment)?),
            ),
            (None, Some(html), Some(attachment)) => message_builder.multipart(
                MultiPart::mixed()
                    .singlepart(Self::text_html(html))
                    .singlepart(Self::attachment(attachment)?),
            ),
            (Some(text), None, None) => message_builder.singlepart(Self::text_plain(text)),
            (None, Some(html), None) => message_builder.singlepart(Self::text_html(html)),
            (None, None, Some(attachment)) => {
                message_builder.singlepart(Self::attachment(attachment)?)
            }
            (None, None, None) => return Err(anyhow!("Missing email body")),
        }
        .context("Can't create MIME formatted email")?;

        Ok(Self { message })
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
            .singlepart(Self::text_plain(text))
            .singlepart(Self::text_html(html))
    }
}

impl fmt::Debug for MimeFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            str::from_utf8(&self.message.formatted()).expect("Can't convert from utf8")
        )
    }
}
