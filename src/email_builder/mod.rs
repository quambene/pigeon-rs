mod bulk_email;
mod bulk_receiver;
mod email;
mod message;
mod message_template;
mod mime;
mod receiver;
mod sender;

pub use bulk_email::BulkEmail;
pub use bulk_receiver::BulkReceiver;
pub use email::Email;
pub use message::Message;
pub use message_template::MessageTemplate;
pub use mime::MimeFormat;
pub use receiver::Receiver;
pub use sender::Sender;

pub enum Confirmed {
    Yes,
    No,
}
