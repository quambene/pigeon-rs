mod bulk_email;
mod email;
mod message;
mod mime;
mod receiver;
mod sender;

pub use bulk_email::BulkEmail;
pub use email::Email;
pub use message::{Message, MessageTemplate, Reader};
pub use mime::MimeFormat;
pub use receiver::Receiver;
pub use sender::Sender;

pub enum Confirmed {
    Yes,
    No,
}
