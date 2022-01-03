use std::{fs, path::Path};

use crate::arg;
use anyhow::anyhow;
use clap::ArgMatches;

#[derive(Debug)]
pub struct TextMessage {
    pub text: String,
}

impl TextMessage {
    pub fn read(matches: &ArgMatches) -> Result<Self, anyhow::Error> {
        if matches.is_present(arg::TEXT_FILE) {
            match matches.value_of(arg::TEXT_FILE) {
                Some(text_file) => {
                    let path = Path::new(text_file);
                    println!("Reading text file '{}' ...", path.display());
                    let text = fs::read_to_string(path)?;
                    let message = TextMessage::new(text);

                    if matches.is_present(arg::DISPLAY) {
                        println!("Display message file: {:#?}", message);
                    }

                    Ok(message)
                }
                None => Err(anyhow!("Missing value for argument '{}'", arg::TEXT_FILE)),
            }
        } else {
            Err(anyhow!("Missing argument '{}'", arg::TEXT_FILE))
        }
    }

    pub fn new(text: String) -> Self {
        Self { text }
    }
}
