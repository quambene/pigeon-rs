use super::{MimeFormat, Receiver, Sender};
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
        let sender = Sender::new(matches)?;
        let receiver = Receiver::new(matches)?;
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
