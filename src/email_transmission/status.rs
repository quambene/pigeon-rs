use crate::helper::{format_green, format_red};
use std::fmt;

#[derive(Debug, Clone)]
pub enum Status {
    DryRun,
    SentOk(String),
    SentError(String),
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Status::DryRun => write!(f, "{}", format_green("dry run")),
            Status::SentOk(message_id) => write!(f, "{} {}", format_green("ok"), message_id),
            Status::SentError(err) => write!(f, "{} {}", format_red("FAILED"), err),
        }
    }
}
