use crate::{arg, data_loader::TabularData, email_builder::MessageTemplate};
use anyhow::{anyhow, Result};
use clap::ArgMatches;
use polars::prelude::DataFrame;

#[derive(Debug, Clone)]
pub struct Message {
    pub subject: String,
    pub text: Option<String>,
    pub html: Option<String>,
}

impl Message {
    pub fn new(matches: &ArgMatches) -> Result<Self, anyhow::Error> {
        let message = if matches.is_present(arg::SUBJECT) && matches.is_present(arg::CONTENT) {
            Message::simple(matches)?
        } else if matches.is_present(arg::MESSAGE_FILE) {
            let message_template = MessageTemplate::read(matches)?;
            Message::default(message_template)?
        } else {
            return Err(anyhow!(
                "Missing arguments. Please provide {} and {} or {}",
                arg::SUBJECT,
                arg::CONTENT,
                arg::MESSAGE_FILE,
            ));
        };

        Ok(message)
    }

    pub fn default(message_template: MessageTemplate) -> Result<Self, anyhow::Error> {
        let message = Message {
            subject: message_template.message.subject,
            text: match message_template.message.text {
                text if !text.is_empty() => Some(text),
                text if text.is_empty() => None,
                _ => unreachable!(),
            },
            html: match message_template.message.html {
                html if !html.is_empty() => Some(Self::html_template(html)),
                html if html.is_empty() => None,
                _ => unreachable!(),
            },
        };
        Ok(message)
    }

    pub fn simple(matches: &ArgMatches) -> Result<Self, anyhow::Error> {
        match (
            matches.value_of(arg::SUBJECT),
            matches.value_of(arg::CONTENT),
        ) {
            (Some(subject), Some(content)) => {
                let message = Message {
                    subject: subject.to_string(),
                    text: match content {
                        text if !text.is_empty() => Some(text.to_string()),
                        text if text.is_empty() => None,
                        _ => unreachable!(),
                    },
                    html: None,
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

    pub fn personalize(
        &mut self,
        index: usize,
        df_receiver: &DataFrame,
        columns: &[&str],
    ) -> Result<(), anyhow::Error> {
        for &col_name in columns.iter() {
            let col_value = TabularData::row(index, col_name, &df_receiver)?;
            self.replace(col_name, col_value);
        }

        Ok(())
    }

    fn replace(&mut self, col_name: &str, col_value: &str) {
        self.subject = self
            .subject
            .replace(&format!("{{{}}}", col_name), col_value);

        self.text = match &self.text {
            Some(text) => Some(text.replace(&format!("{{{}}}", col_name), col_value)),
            None => None,
        };

        self.html = match &self.html {
            Some(html) => Some(html.replace(&format!("{{{}}}", col_name), col_value)),
            None => None,
        };
    }

    fn html_template(content: String) -> String {
        format!(
            "<html>
            <head></head>
            <body>
            {}
            </body>
            </html>",
            content
        )
    }
}
