use crate::email_builder::{Email, Message, Status};

#[derive(Debug)]
pub struct SentEmail {
    pub sender: String,
    pub receiver: String,
    pub message: Message,
    pub status: Status,
}

impl SentEmail {
    pub fn new(email: &Email, status: Status) -> Self {
        Self {
            sender: email.sender.clone(),
            receiver: email.receiver.clone(),
            message: email.message.clone(),
            status,
        }
    }

    pub fn display_status(&self) {
        println!("{:#?} ... {:#?}", self.receiver, self.status);
    }
}
