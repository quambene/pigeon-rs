use crate::email_builder;
use anyhow::{anyhow, Context};
use lettre::{
    message::{header, MultiPart, SinglePart},
    Message as LettreMessage,
};
use std::{fmt, fs, path::Path, str, time::SystemTime};

#[derive(Clone)]
pub struct MimeFormat {
    pub message: LettreMessage,
}

impl MimeFormat {
    pub fn new(
        sender: &str,
        receiver: &str,
        message: &email_builder::Message,
        attachment: Option<&str>,
        now: SystemTime,
    ) -> Result<Self, anyhow::Error> {
        let message_builder = LettreMessage::builder()
            .from(sender.parse().context("Can't parse sender")?)
            .to(receiver.parse().context("Can't parse receiver")?)
            .subject(&message.subject)
            .date(now);

        let message = match (&message.text, &message.html, attachment) {
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
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            str::from_utf8(&self.message.formatted()).expect("Can't convert from utf8")
        )
    }
}

#[cfg(test)]
mod tests {
    use self::email_builder::Message;
    use super::*;
    use std::{fs::File, io::Read, time::UNIX_EPOCH};

    #[test]
    fn test_mime_format_plaintext() {
        let date_time = chrono::DateTime::parse_from_rfc3339("2024-01-01T14:00:00Z")
            .unwrap()
            .timestamp() as u64;
        let system_time = UNIX_EPOCH + std::time::Duration::from_secs(date_time);
        let sender = "albert@einstein.com";
        let receiver = "marie@curie.com";
        let subject = "Test Subject";
        let text = "This is a test message (plaintext).";
        let message = Message::new(subject, Some(text), None);

        let res = MimeFormat::new(sender, receiver, &message, None, system_time);
        assert!(res.is_ok());

        let mime_format = format!("{:?}", res.unwrap());
        let mut expected_file = File::open("./test_data/email_plaintext.txt").unwrap();
        let mut expected_format = String::new();
        expected_file.read_to_string(&mut expected_format).unwrap();
        assert_eq!(mime_format.replace("\r", ""), expected_format);
    }

    #[test]
    fn test_mime_format_html() {
        let date_time = chrono::DateTime::parse_from_rfc3339("2024-01-01T14:00:00Z")
            .unwrap()
            .timestamp() as u64;
        let system_time = UNIX_EPOCH + std::time::Duration::from_secs(date_time);
        let sender = "albert@einstein.com";
        let receiver = "marie@curie.com";
        let subject = "Test Subject";
        let html = "<p>This is a test message (html).</p>";
        let message = Message::new(subject, None, Some(html));

        let res = MimeFormat::new(sender, receiver, &message, None, system_time);
        assert!(res.is_ok());

        let mime_format = format!("{:?}", res.unwrap());
        let mut expected_file = File::open("./test_data/email_html.txt").unwrap();
        let mut expected_format = String::new();
        expected_file.read_to_string(&mut expected_format).unwrap();
        assert_eq!(mime_format.replace("\r", ""), expected_format);
    }
}
