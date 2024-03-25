mod message_template;
mod reader;

pub use self::{message_template::MessageTemplate, reader::Reader};
use crate::{arg, data_loader::TabularData};
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
    pub fn new<S>(subject: S, text: Option<S>, html: Option<S>) -> Self
    where
        S: Into<String>,
    {
        Self {
            subject: subject.into(),
            text: text.map(|text| text.into()),
            html: html.map(|text| text.into()),
        }
    }

    pub fn build(matches: &ArgMatches) -> Result<Self, anyhow::Error> {
        let message = if matches.is_present(arg::SUBJECT) && matches.is_present(arg::CONTENT) {
            Message::from_cmd(matches)?
        } else if matches.is_present(arg::MESSAGE_FILE) {
            Message::from_template(matches)?
        } else if matches.is_present(arg::SUBJECT)
            && (matches.is_present(arg::TEXT_FILE) || matches.is_present(arg::HTML_FILE))
        {
            Message::from_file(matches)?
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

    pub fn from_file(matches: &ArgMatches) -> Result<Self, anyhow::Error> {
        let subject = Message::subject(matches)?.to_string();
        let text = Reader::read_txt(matches)?;
        let html = Reader::read_html(matches)?;
        let message = Message::new(subject, text, html);
        Ok(message)
    }

    pub fn from_template(matches: &ArgMatches) -> Result<Self, anyhow::Error> {
        let message_template = MessageTemplate::read(matches)?;
        let message = Message::new(
            message_template.subject,
            message_template.text,
            message_template.html,
        );
        Ok(message)
    }

    pub fn from_cmd(matches: &ArgMatches) -> Result<Self, anyhow::Error> {
        match (
            matches.value_of(arg::SUBJECT),
            matches.value_of(arg::CONTENT),
        ) {
            (Some(subject), Some(content)) => {
                let message = Message::new(subject, Some(content), None);
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
            let col_value = TabularData::row(index, col_name, df_receiver)?;
            self.replace(col_name, col_value);
        }

        Ok(())
    }

    fn replace(&mut self, col_name: &str, col_value: &str) {
        self.subject = self
            .subject
            .replace(&format!("{{{}}}", col_name), col_value);
        self.text = self
            .text
            .as_ref()
            .map(|text| text.replace(&format!("{{{}}}", col_name), col_value));
        self.html = self
            .html
            .as_ref()
            .map(|html| html.replace(&format!("{{{}}}", col_name), col_value));
    }

    fn subject<'a>(matches: &'a ArgMatches) -> Result<&'a str, anyhow::Error> {
        if matches.is_present(arg::SUBJECT) {
            match matches.value_of(arg::SUBJECT) {
                Some(subject) => Ok(subject),
                None => Err(anyhow!("Missing value for argument '{}'", arg::SUBJECT)),
            }
        } else {
            Err(anyhow!("Missing argument '{}'", arg::SUBJECT))
        }
    }
}
