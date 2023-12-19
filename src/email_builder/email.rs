use crate::{
    arg,
    email_builder::{Confirmed, Message, MimeFormat, Receiver, Sender},
};
use anyhow::{anyhow, Context, Result};
use clap::ArgMatches;
use std::io;

#[derive(Debug)]
pub struct Email<'a> {
    pub sender: &'a str,
    pub receiver: &'a str,
    pub message: Message,
    pub mime_format: MimeFormat,
}

impl<'a> Email<'a> {
    pub fn build(matches: &'a ArgMatches) -> Result<Self, anyhow::Error> {
        let sender = Sender::init(matches)?;
        let receiver = Receiver::init(matches)?;
        let message = Message::build(matches)?;
        let mime_format = MimeFormat::new(matches, sender, receiver, &message)?;
        let email = Email::new(sender, receiver, &message, &mime_format)?;
        Ok(email)
    }

    pub fn new(
        sender: &'a str,
        receiver: &'a str,
        message: &Message,
        mime_format: &MimeFormat,
    ) -> Result<Self, anyhow::Error> {
        let email = Email {
            sender,
            receiver,
            message: message.to_owned(),
            mime_format: mime_format.to_owned(),
        };
        Ok(email)
    }

    pub fn confirm(&self, matches: &ArgMatches) -> Result<Confirmed, anyhow::Error> {
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
