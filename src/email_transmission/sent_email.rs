use crate::email_builder::{Email, Message, Status};

#[derive(Debug)]
pub struct SentEmail<'a> {
    pub sender: &'a str,
    pub receiver: String,
    pub message: Message,
    pub status: Status,
}

impl<'a> SentEmail<'a> {
    pub fn new(email: &Email<'a>, status: Status) -> Self {
        Self {
            sender: email.sender,
            receiver: email.receiver.to_string(),
            message: email.message.clone(),
            status,
        }
    }

    pub fn display_status(&self) {
        println!("{} ... {}", self.receiver, self.status);
    }
}
