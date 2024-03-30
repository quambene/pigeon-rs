mod bulk_email;
mod email;
mod message;
mod mime;
mod receiver;
mod sender;

pub use bulk_email::BulkEmail;
pub use email::Email;
pub use message::Message;
pub use mime::MimeFormat;
pub use receiver::{BulkReceiver, Receiver};
pub use sender::Sender;

pub enum Confirmed {
    Yes,
    No,
}
