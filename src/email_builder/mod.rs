mod bulk_email;
mod email;
mod message;
mod message_template;
mod mime;

pub use bulk_email::BulkEmail;
pub use email::Email;
pub use message::Message;
pub use message_template::MessageTemplate;
pub use mime::MimeFormat;

pub enum Confirmed {
    Yes,
    No,
}
