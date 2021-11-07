use crate::{arg, email_handler::Message};
use anyhow::{anyhow, Context, Result};
use clap::ArgMatches;
use serde::Deserialize;
use std::{
    env, fs,
    io::{self, Write},
    path::PathBuf,
    time::SystemTime,
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

fn create_template(path: PathBuf) -> Result<(), anyhow::Error> {
    let mut message_template = fs::File::create(path).expect("Unable to create message template.");
    message_template.write_all(MESSAGE_TEMPLATE.as_bytes())?;
    Ok(())
}

impl MessageTemplate {
    pub fn create(_matches: &ArgMatches<'_>) -> Result<(), anyhow::Error> {
        let current_dir = env::current_dir().expect("Can't get current directory");
        let path_dir = current_dir;

        let file_name = TEMPLATE_FILE_NAME;
        let template_path = path_dir.join(file_name);

        match template_path.exists() {
            true => {
                let mut input = String::new();
                println!("Message template already exists. Should this template be overwritten? Yes (y) or no (n)");
                loop {
                    io::stdin().read_line(&mut input).expect("Can't read input");
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

    pub fn read(matches: &ArgMatches<'_>) -> Result<Self, anyhow::Error> {
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

    pub fn archive(matches: &ArgMatches<'_>) -> Result<(), anyhow::Error> {
        let now = SystemTime::now();
        let now_utc: chrono::DateTime<chrono::Utc> = now.into();
        let current_time = now_utc.to_rfc3339_opts(chrono::SecondsFormat::Secs, true);
        let source_path: &str;

        match matches.value_of(arg::MESSAGE_FILE) {
            Some(message_file) => {
                source_path = message_file;
            }
            None => {
                return Err(anyhow!(
                    "Missing value for argument '{}'",
                    arg::MESSAGE_FILE
                ));
            }
        }

        let target_dir = PathBuf::from("./archived_templates");
        let target_file: String;

        if matches.is_present(arg::DRY_RUN) {
            target_file = String::from("message_") + &current_time + "_dry-run" + ".yaml";
        } else {
            target_file = String::from("message_") + &current_time + ".yaml";
        }

        let target_path = target_dir.join(target_file);

        if target_dir.exists() {
            println!("Archiving template file ...");
            fs::copy(source_path, target_path).context("Unable to archive mail.")?;
        } else {
            println!(
                "Creating directory for archived templates: {:#?}",
                target_dir
            );
            fs::create_dir(target_dir)
                .context("Unable to create directory for archived templates.")?;
            println!("Archiving template file ...");
            fs::copy(source_path, target_path).context("Unable to archive mail.")?;
        }

        Ok(())
    }
}
