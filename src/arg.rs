use anyhow::anyhow;
use clap::ArgMatches;

// args for command
pub const VERBOSE: &str = "verbose";

// args for subcommands
pub const SENDER: &str = "sender";
pub const RECEIVER: &str = "receiver";
pub const SUBJECT: &str = "subject";
pub const CONTENT: &str = "content";
pub const DRY_RUN: &str = "dry-run";
pub const DISPLAY: &str = "display";
pub const MESSAGE_FILE: &str = "message-file";
pub const TEXT_FILE: &str = "text-file";
pub const HTML_FILE: &str = "html-file";
pub const ARCHIVE: &str = "archive";
pub const ARCHIVE_DIR: &str = "archive-dir";
pub const RECEIVER_FILE: &str = "receiver-file";
pub const RECEIVER_QUERY: &str = "receiver-query";
pub const RECEIVER_COLUMN: &str = "receiver-column";
pub const ASSUME_YES: &str = "assume-yes";
pub const PERSONALIZE: &str = "personalize";
pub const ATTACHMENT: &str = "attachment";
pub const SAVE: &str = "save";
pub const SAVE_DIR: &str = "save-dir";
pub const FILE_TYPE: &str = "file-type";
pub const IMAGE_COLUMN: &str = "image-column";
pub const IMAGE_NAME: &str = "image-name";
pub const SSH_TUNNEL: &str = "ssh-tunnel";
pub const CONNECTION: &str = "connection";

// values for args
pub mod val {
    // default value for RECEIVER_COLUMN
    pub const EMAIL: &str = "email";

    // possible values for argument CONNECTION and subcommand CONNECT
    pub const SMTP: &str = "smtp";
    pub const AWS: &str = "aws";
}

pub fn value<'a>(name: &str, matches: &'a ArgMatches) -> Result<&'a str, anyhow::Error> {
    match matches.get_one::<String>(name) {
        Some(query) => Ok(query),
        None => Err(anyhow!("Missing value for argument '{}'", name)),
    }
}
