mod client;
mod mock_client;
mod sent_email;
mod smtp;
mod status;

pub use client::Client;
pub use mock_client::MockClient;
pub use sent_email::SentEmail;
pub use smtp::SmtpClient;
pub use status::Status;

use crate::email_builder::Email;

pub trait SendEmail<'a> {
    fn send(&self, email: &'a Email<'a>) -> Result<SentEmail<'a>, anyhow::Error>;
}
