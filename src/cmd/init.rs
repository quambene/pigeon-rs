use crate::{arg, email_builder::Message};
use anyhow::Context;
use clap::ArgMatches;
use std::{env, io};

pub fn init(matches: &ArgMatches) -> Result<(), anyhow::Error> {
    if matches.is_present(arg::VERBOSE) {
        println!("matches: {:#?}", matches);
    }

    let current_dir = env::current_dir().context("Can't get current directory")?;
    let file_name = Message::template_name();
    let template_path = current_dir.join(file_name);

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
                            "Overwriting message template in current directory '{}' ...",
                            template_path.display()
                        );
                        Message::write_template(&template_path)?;
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
                "Creating message template in current directory: {} ...",
                template_path.display()
            );
            Message::write_template(&template_path)?;
        }
    }

    Ok(())
}
