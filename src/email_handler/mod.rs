mod bulk_email;
mod email;
mod message;
mod message_template;

pub use bulk_email::BulkEmail;
pub use email::Email;
pub use message::Message;
pub use message_template::MessageTemplate;

pub enum Confirmed {
    Yes,
    No,
}