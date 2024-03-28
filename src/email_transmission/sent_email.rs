use super::Status;
use crate::email_builder::{Email, Message, Sender};

#[derive(Debug)]
pub struct SentEmail<'a> {
    pub sender: &'a Sender<'a>,
    pub receiver: &'a str,
    pub message: &'a Message,
    pub status: Status,
}

impl<'a> SentEmail<'a> {
    pub fn new(email: &'a Email<'a>, status: Status) -> Self {
        Self {
            sender: email.sender,
            receiver: email.receiver,
            message: &email.message,
            status,
        }
    }

    pub fn display_status(&self) {
        println!("{} ... {}", self.receiver, self.status);
    }
}
