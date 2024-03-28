use super::{Receiver, Sender};
use crate::{
    arg,
    email_builder::{Confirmed, Message, MimeFormat},
};
use anyhow::{anyhow, Context, Result};
use clap::ArgMatches;
use std::io;

#[derive(Debug)]
pub struct Email<'a> {
    pub sender: Sender<'a>,
    pub receiver: Receiver<'a>,
    pub message: Message,
    pub mime_format: MimeFormat,
}

impl<'a> Email<'a> {
    pub fn new(
        sender: Sender<'a>,
        receiver: Receiver<'a>,
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
