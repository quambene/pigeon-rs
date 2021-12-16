use crate::{
    arg,
    email_handler::{Confirmed, Message, MessageTemplate},
    email_provider,
    helper::{check_send_status, format_green},
};
use anyhow::{anyhow, Context, Result};
use clap::ArgMatches;
use std::io;

#[derive(Debug)]
pub struct Email {
    pub sender: String,
    pub receiver: String,
    pub message: Message,
}

impl Email {
    pub fn new(matches: &ArgMatches<'_>) -> Result<Self, anyhow::Error> {
        match (
            matches.value_of(arg::SENDER),
            matches.value_of(arg::RECEIVER),
        ) {
            (Some(sender), Some(receiver)) => {
                let message = Message::new(matches)?;
                let email = Email {
                    sender: sender.to_string(),
                    receiver: receiver.to_string(),
                    message,
                };
                Ok(email)
            }
            (Some(_), None) => Err(anyhow!("Missing value for argument '{}'", arg::RECEIVER)),
            (None, Some(_)) => Err(anyhow!("Missing value for argument '{}'", arg::SENDER)),
            (None, None) => Err(anyhow!(
                "Missing values for arguments '{}' and '{}'",
                arg::SENDER,
                arg::RECEIVER
            )),
        }
    }

    pub fn send(&self, matches: &ArgMatches<'_>) -> Result<(), anyhow::Error> {
        println!("Sending email to 1 recipient ...");
        match matches.is_present(arg::DRY_RUN) {
            true => {
                // Setup client but do not send email
                let _client = email_provider::setup_ses_client(matches)?;
                println!("{:#?} ... {}", self.receiver, format_green("dry run"));
                println!("All emails sent (dry run).");
                Ok(())
            }
            false => {
                let client = email_provider::setup_ses_client(matches)?;
                let res = email_provider::send_email(self, &client);
                let status = check_send_status(res);
                println!("{:#?} ... {}", self.receiver, status);
                println!("All emails sent.");
                Ok(())
            }
        }
    }

    pub fn confirm(&self, matches: &ArgMatches<'_>) -> Result<Confirmed, anyhow::Error> {
        let mut input = String::new();

        match matches.value_of(arg::RECEIVER) {
            Some(receiver) => println!("Preparing to send an email to 1 recipient: {}", receiver),
            None => return Err(anyhow!("Missing value for argument '{}'", arg::RECEIVER)),
        }

        println!("Should an email be sent to 1 recipient? Yes (y) or no (n)");
        let confirmation = loop {
            io::stdin()
                .read_line(&mut input)
                .context("Can't read input")?;
            match input.trim() {
                "y" | "yes" | "Yes" => {
                    break Confirmed::Yes;
                }
                "n" | "no" | "No" => {
                    println!("Aborted ...");
                    break Confirmed::No;
                }
                _ => {
                    println!("Choose yes (y) or no (n). Try again.");
                    continue;
                }
            }
        };
        Ok(confirmation)
    }

    pub fn archive(&self, matches: &ArgMatches<'_>) -> Result<(), anyhow::Error> {
        if matches.is_present(arg::MESSAGE_FILE) {
            MessageTemplate::archive(matches)?;
        }

        Ok(())
    }
}
