use crate::arg;
use anyhow::{anyhow, Context, Result};
use clap::ArgMatches;
use serde::Deserialize;
use std::{
    fs,
    io::Write,
    path::{Path, PathBuf},
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

#[derive(Debug, Deserialize)]
pub struct MessageTemplate {
    pub subject: String,
    pub text: Option<String>,
    pub html: Option<String>,
}

impl MessageTemplate {
    pub fn file_name() -> &'static str {
        TEMPLATE_FILE_NAME
    }

    pub fn read(matches: &ArgMatches) -> Result<Self, anyhow::Error> {
        if matches.is_present(arg::MESSAGE_FILE) {
            match matches.value_of(arg::MESSAGE_FILE) {
                Some(message_file) => {
                    let path = PathBuf::from(message_file);
                    println!("Reading message file '{}' ...", path.display());
                    let yaml = fs::read_to_string(&path)?;
                    let message = serde_yaml::from_str(&yaml)?;

                    if matches.is_present(arg::DISPLAY) {
                        println!("Display message file: {:#?}", message);
                    }

                    Ok(message)
                }
                None => Err(anyhow!(
                    "Missing value for argument '{}'",
                    arg::MESSAGE_FILE
                )),
            }
        } else {
            Err(anyhow!("Missing argument '{}'", arg::MESSAGE_FILE))
        }
    }

    pub fn write(path: &Path) -> Result<(), anyhow::Error> {
        let mut message_template =
            fs::File::create(path).context("Unable to create message template.")?;
        message_template.write_all(MESSAGE_TEMPLATE.as_bytes())?;
        Ok(())
    }
}
