use crate::email_builder::{Email, Status};

#[derive(Debug)]
pub struct SentEmail {
    pub email: Email,
    pub status: Status,
}

impl SentEmail {
    pub fn new(email: &Email, status: Status) -> Self {
        Self {
            email: email.clone(),
            status,
        }
    }

    pub fn display_status(&self) {
        println!("{:#?} ... {:#?}", self.email.receiver, self.status);
    }
}
