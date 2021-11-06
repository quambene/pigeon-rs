use crate::{arg, email_handler::MessageTemplate};
use anyhow::{anyhow, Result};
use clap::ArgMatches;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Message {
    pub subject: String,
    pub text: String,
    pub html: String,
}

impl Message {
    pub fn new(matches: &ArgMatches<'_>) -> Result<Self, anyhow::Error> {
        if matches.is_present(arg::SUBJECT) && matches.is_present(arg::CONTENT) {
            Message::simple(matches)
        } else if matches.is_present(arg::MESSAGE_FILE) {
            let message_template = MessageTemplate::read(matches)?;
            Message::default(&message_template)
        } else {
            Err(anyhow!(
                "Missing arguments. Please provide {} and {} or {}",
                arg::SUBJECT,
                arg::CONTENT,
                arg::MESSAGE_FILE
            ))
        }
    }

    pub fn default(message_template: &MessageTemplate) -> Result<Self, anyhow::Error> {
        let message = Message {
            subject: message_template.message.subject.to_string(),
            text: message_template.message.text.to_string(),
            html: "<html>
                    <head></head>
                    <body>"
                .to_string()
                + &message_template.message.html
                + "</body>
                    </html>",
        };
        Ok(message)
    }

    fn simple(matches: &ArgMatches<'_>) -> Result<Self, anyhow::Error> {
        match (
            matches.value_of(arg::SUBJECT),
            matches.value_of(arg::CONTENT),
        ) {
            (Some(subject), Some(content)) => {
                let message = Message {
                    subject: subject.to_string(),
                    text: content.to_string(),
                    html: "<html>
                            <head></head>
                            <body>"
                        .to_string()
                        + &content.to_string()
                        + "</body>
                            </html>",
                };
                Ok(message)
            }
            (Some(_), None) => Err(anyhow!("Missing value for argument '{}'", arg::CONTENT)),
            (None, Some(_)) => Err(anyhow!("Missing value for argument '{}'", arg::SUBJECT)),
            (None, None) => Err(anyhow!(
                "Missing values for '{}' and '{}'",
                arg::SUBJECT,
                arg::CONTENT
            )),
        }
    }

    pub fn personalize(&self, col_name: &str, col_value: &str) -> Result<Self, anyhow::Error> {
        let message = Message {
            subject: self
                .subject
                .replace(&format!("{{{}}}", col_name), col_value),
            text: self.text.replace(&format!("{{{}}}", col_name), col_value),
            html: self.html.replace(&format!("{{{}}}", col_name), col_value),
        };

        Ok(message)
    }
}
