use super::MimeFormat;
use crate::{
    arg,
    email_builder::{Confirmed, Message},
};
use anyhow::{anyhow, Context, Result};
use clap::ArgMatches;
use std::io;

#[derive(Debug, Clone)]
pub struct Email<'a> {
    pub sender: &'a str,
    pub receiver: String,
    pub message: Message,
    pub mime_format: MimeFormat,
}

impl<'a> Email<'a> {
    pub fn new(matches: &'a ArgMatches<'a>) -> Result<Self, anyhow::Error> {
        match (
            matches.value_of(arg::SENDER),
            matches.value_of(arg::RECEIVER),
        ) {
            (Some(sender), Some(receiver)) => {
                let message = Message::new(matches)?;
                let mime_format = MimeFormat::new(matches, sender, receiver, &message)?;
                let email = Email {
                    sender,
                    receiver: receiver.to_string(),
                    message,
                    mime_format,
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
}
