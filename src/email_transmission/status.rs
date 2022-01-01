use std::fmt;

use crate::helper::{format_green, format_red};

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
            Status::SentOk(ok) => {
                let messages: Vec<&str> = ok.split(' ').collect();
                let ok_string = messages[0];
                let message_id = messages[1];
                write!(f, "{} {}", format_green(ok_string), message_id)
            }
            Status::SentError(err) => write!(f, "{} {}", format_red("FAILED"), err),
        }
    }
}
