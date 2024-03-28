use super::{Receiver, Sender};
use crate::email_builder::{Message, MimeFormat};

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
}
