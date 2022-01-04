mod client;
mod sent_email;
mod smtp;
mod status;

pub use client::{Client, SendEmail};
pub use sent_email::SentEmail;
pub use smtp::SmtpClient;
pub use status::Status;
