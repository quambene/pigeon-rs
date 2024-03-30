use super::MessageTemplate;
use crate::arg;
use anyhow::{anyhow, Result};
use clap::ArgMatches;
use std::{fs, path::Path};

#[derive(Debug, Clone, PartialEq)]
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

    pub fn from_args(matches: &ArgMatches) -> Result<Self, anyhow::Error> {
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
        let text = Self::read(matches, arg::TEXT_FILE)?;
        let html = Self::read(matches, arg::HTML_FILE)?;
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

    pub fn personalize(&mut self, col_name: &str, col_value: &str) {
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

    fn read(matches: &ArgMatches, arg: &str) -> Result<Option<String>, anyhow::Error> {
        if matches.is_present(arg) {
            match matches.value_of(arg) {
                Some(text_file) => {
                    let path = Path::new(text_file);
                    println!("Reading text file '{}' ...", path.display());
                    let message = fs::read_to_string(path)?;

                    if matches.is_present(arg::DISPLAY) {
                        println!("Display message file: {:#?}", message);
                    }

                    Ok(Some(message))
                }
                None => Err(anyhow!("Missing value for argument '{}'", arg)),
            }
        } else {
            Ok(None)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::app;

    #[test]
    fn test_message_from_args_subject_content() {
        let args = vec![
            "pigeon",
            "send",
            "albert@einstein.com",
            "marie@curie.com",
            "--subject",
            "Test subject",
            "--content",
            "This is a test message (plaintext).",
        ];
        let app = app();
        let matches = app.get_matches_from(args);
        let subcommand_matches = matches.subcommand_matches("send").unwrap();

        let res = Message::from_args(subcommand_matches);
        assert!(res.is_ok(), "{}", res.unwrap_err());

        let message = res.unwrap();
        assert_eq!(
            message,
            Message {
                subject: "Test subject".to_owned(),
                text: Some("This is a test message (plaintext).".to_owned()),
                html: None,
            }
        );
    }

    #[test]
    fn test_message_from_args_text_and_html_file() {
        let args = vec![
            "pigeon",
            "send",
            "albert@einstein.com",
            "marie@curie.com",
            "--subject",
            "Test subject",
            "--text-file",
            "./test_data/message.txt",
            "--html-file",
            "./test_data/message.html",
        ];
        let app = app();
        let matches = app.get_matches_from(args);
        let subcommand_matches = matches.subcommand_matches("send").unwrap();

        let res = Message::from_args(subcommand_matches);
        assert!(res.is_ok(), "{}", res.unwrap_err());

        let message = res.unwrap();
        assert_eq!(
            message,
            Message {
                subject: "Test subject".to_owned(),
                text: Some(
                    "This is a test message (plaintext).\n\nThis is the last line.".to_owned()
                ),
                html: Some(
                    "<p>This is a test message (html).</p>\n\n<p>This is the last line.</p>"
                        .to_owned()
                ),
            }
        );
    }

    #[test]
    fn test_message_from_args_message_file() {
        let args = vec![
            "pigeon",
            "send",
            "albert@einstein.com",
            "marie@curie.com",
            "--message-file",
            "./test_data/message.yaml",
        ];
        let app = app();
        let matches = app.get_matches_from(args);
        let subcommand_matches = matches.subcommand_matches("send").unwrap();

        let res = Message::from_args(subcommand_matches);
        assert!(res.is_ok(), "{}", res.unwrap_err());

        let message = res.unwrap();
        assert_eq!(
            message,
            Message {
                subject: "Test subject".to_owned(),
                text: Some("This is a test message (plaintext).".to_owned()),
                html: Some("<p>This is a test message (html).</p>".to_owned()),
            }
        );
    }

    #[test]
    fn test_message_from_args_message_file_empty() {
        let args = vec![
            "pigeon",
            "send",
            "albert@einstein.com",
            "marie@curie.com",
            "--message-file",
            "./test_data/message_empty.yaml",
        ];
        let app = app();
        let matches = app.get_matches_from(args);
        let subcommand_matches = matches.subcommand_matches("send").unwrap();

        let res = Message::from_args(subcommand_matches);
        assert!(res.is_ok(), "{}", res.unwrap_err());

        let message = res.unwrap();
        assert_eq!(
            message,
            Message {
                subject: "Test subject".to_owned(),
                text: Some("".to_owned()),
                html: Some("".to_owned()),
            }
        );
    }

    #[test]
    fn test_message_from_args_message_none() {
        let args = vec![
            "pigeon",
            "send",
            "albert@einstein.com",
            "marie@curie.com",
            "--message-file",
            "./test_data/message_none.yaml",
        ];
        let app = app();
        let matches = app.get_matches_from(args);
        let subcommand_matches = matches.subcommand_matches("send").unwrap();

        let res = Message::from_args(subcommand_matches);
        assert!(res.is_ok(), "{}", res.unwrap_err());

        let message = res.unwrap();
        assert_eq!(
            message,
            Message {
                subject: "Test subject".to_owned(),
                text: None,
                html: None,
            }
        );
    }
}
