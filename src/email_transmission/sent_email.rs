use super::Status;
use crate::email_builder::{Email, Message, Receiver, Sender};

#[derive(Debug)]
pub struct SentEmail<'a> {
    #[allow(dead_code)]
    pub sender: Sender<'a>,
    pub receiver: Receiver<'a>,
    #[allow(dead_code)]
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
        println!("{} ... {}", self.receiver.0, self.status);
    }
}
