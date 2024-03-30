use crate::{arg, utils};
use anyhow::{anyhow, Context, Result};
use clap::ArgMatches;
use serde::Deserialize;
use std::{
    fs::{self, File},
    io::Write,
    path::Path,
};

const TEMPLATE_FILE_NAME: &str = "message.yaml";

static MESSAGE_TEMPLATE: &str = r##"# Specify the subject, plaintext and html version of your email.
# Personalize message by wrapping variables in curly brackets, eg. {first_name}.

# The subject of your email
subject: ""
# The plaintext version
text: ""
# The html version
html: ""
"##;

#[derive(Debug, Clone, PartialEq, Deserialize)]
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
        if matches.is_present(arg::SUBJECT) && matches.is_present(arg::CONTENT) {
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
        } else if matches.is_present(arg::MESSAGE_FILE) {
            let message_file = arg::value(arg::MESSAGE_FILE, matches)?;
            let message_path = Path::new(message_file);
            let message = Message::read_yaml(message_path)?;

            if matches.is_present(arg::DISPLAY) {
                println!("Display message file: {:#?}", message);
            }

            Ok(message)
        } else if matches.is_present(arg::SUBJECT)
            && (matches.is_present(arg::TEXT_FILE) || matches.is_present(arg::HTML_FILE))
        {
            let subject = arg::value(arg::SUBJECT, matches)?;
            let text_path = matches.value_of(arg::TEXT_FILE).map(Path::new);
            let html_path = matches.value_of(arg::HTML_FILE).map(Path::new);
            let text = if let Some(path) = text_path {
                Some(utils::read_file(path)?)
            } else {
                None
            };
            let html = if let Some(path) = html_path {
                Some(utils::read_file(path)?)
            } else {
                None
            };
            let message = Message::new(subject, text.as_deref(), html.as_deref());
            Ok(message)
        } else {
            Err(anyhow!(
                "Missing arguments. Please provide {} and {} or {}",
                arg::SUBJECT,
                arg::CONTENT,
                arg::MESSAGE_FILE,
            ))
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

    fn read_yaml(path: &Path) -> Result<Self, anyhow::Error> {
        println!("Reading message file '{}' ...", path.display());
        let yaml = fs::read_to_string(&path)?;
        let message = serde_yaml::from_str(&yaml)?;
        Ok(message)
    }

    pub fn template_name() -> &'static str {
        TEMPLATE_FILE_NAME
    }

    pub fn write_template(path: &Path) -> Result<(), anyhow::Error> {
        let mut message_file = File::create(path).context("Unable to create message template.")?;
        message_file.write_all(MESSAGE_TEMPLATE.as_bytes())?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::app;

    #[test]
    fn test_read_yaml() {
        let yaml_path = Path::new("./test_data/message.yaml");
        let res = Message::read_yaml(yaml_path);
        assert!(res.is_ok());

        let message = res.unwrap();
        assert_eq!(
            message,
            Message {
                subject: "Test subject".to_owned(),
                text: Some("This is a test message (plaintext).".to_owned()),
                html: Some("<p>This is a test message (html).</p>".to_owned())
            }
        )
    }

    #[test]
    fn test_read_yaml_empty() {
        let yaml_path = Path::new("./test_data/message_empty.yaml");
        let res = Message::read_yaml(yaml_path);
        assert!(res.is_ok());

        let message = res.unwrap();
        assert_eq!(
            message,
            Message {
                subject: "Test subject".to_owned(),
                text: Some("".to_owned()),
                html: Some("".to_owned())
            }
        )
    }

    #[test]
    fn test_read_yaml_none() {
        let yaml_path = Path::new("./test_data/message_none.yaml");
        let res = Message::read_yaml(yaml_path);
        assert!(res.is_ok());

        let message = res.unwrap();
        assert_eq!(
            message,
            Message {
                subject: "Test subject".to_owned(),
                text: None,
                html: None,
            }
        )
    }

    #[test]
    fn test_personalize() {
        let text = r#"Dear {first_name} {last_name},
This is a test message (plaintext)."#;
        let html = r#"Dear {first_name} {last_name},
<br>
<br>
This is a test message (html)."#;
        let mut message = Message::new(
            "Test subject".to_owned(),
            Some(text.to_owned()),
            Some(html.to_owned()),
        );
        message.personalize("first_name", "Marie");
        message.personalize("last_name", "Curie");
        assert_eq!(
            message,
            Message {
                subject: "Test subject".to_owned(),
                text: Some("Dear Marie Curie,\nThis is a test message (plaintext).".to_owned()),
                html: Some(
                    "Dear Marie Curie,\n<br>\n<br>\nThis is a test message (html).".to_owned()
                )
            }
        );
    }

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
}
