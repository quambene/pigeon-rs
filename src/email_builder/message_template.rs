use crate::arg;
use anyhow::{anyhow, Context, Result};
use clap::ArgMatches;
use serde::Deserialize;
use std::{
    env, fs,
    io::{self, Write},
    path::PathBuf,
};

const TEMPLATE_FILE_NAME: &str = "message.yaml";
static MESSAGE_TEMPLATE: &str = r##"# You can leave EITHER the text OR the html empty, but not both. Ideally, fill out both.
# You MUST provide a subject. Personalize message by wrapping variables in curly brackets, eg. {firstname}.

message:
    # The subject of your email
    subject: ""
    # The plaintext version
    text: ""
    # The html version
    html: ""
"##;

#[derive(Debug, Deserialize)]
pub struct MessageTemplate {
    pub message: Message,
}

#[derive(Debug, Deserialize)]
pub struct Message {
    pub subject: String,
    pub text: String,
    pub html: String,
}

impl MessageTemplate {
    pub fn create(_matches: &ArgMatches) -> Result<(), anyhow::Error> {
        let current_dir = env::current_dir().context("Can't get current directory")?;
        let path_dir = current_dir;

        let file_name = TEMPLATE_FILE_NAME;
        let template_path = path_dir.join(file_name);

        match template_path.exists() {
            true => {
                let mut input = String::new();
                println!("Message template already exists. Should this template be overwritten? Yes (y) or no (n)");
                loop {
                    io::stdin()
                        .read_line(&mut input)
                        .context("Can't read input")?;
                    match input.trim() {
                        "y" | "yes" | "Yes" => {
                            println!(
                                "Overwriting message template in current directory '{:#?}' ...",
                                template_path
                            );
                            create_template(template_path)?;
                            break;
                        }
                        "n" | "no" | "No" => {
                            println!("Aborted ...");
                            break;
                        }
                        _ => {
                            println!("Choose yes (y) or no (n). Try again.");
                            continue;
                        }
                    }
                }
            }
            false => {
                println!(
                    "Creating message template in current directory: {:#?} ...",
                    template_path
                );
                create_template(template_path)?;
            }
        }

        Ok(())
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
}

fn create_template(path: PathBuf) -> Result<(), anyhow::Error> {
    let mut message_template =
        fs::File::create(path).context("Unable to create message template.")?;
    message_template.write_all(MESSAGE_TEMPLATE.as_bytes())?;
    Ok(())
}
