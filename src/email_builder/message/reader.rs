use crate::arg;
use anyhow::anyhow;
use clap::ArgMatches;
use std::{fs, path::Path};

#[derive(Debug)]
pub struct Reader;

impl Reader {
    pub fn read_txt(matches: &ArgMatches) -> Result<Option<String>, anyhow::Error> {
        Self::read(matches, arg::TEXT_FILE)
    }

    pub fn read_html(matches: &ArgMatches) -> Result<Option<String>, anyhow::Error> {
        Self::read(matches, arg::HTML_FILE)
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
