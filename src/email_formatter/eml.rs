use crate::{arg, email_builder::Email};
use anyhow::{anyhow, Context};
use clap::ArgMatches;
use lettre::{FileTransport, Transport};
use std::{
    fs,
    path::{Path, PathBuf},
};

pub struct EmlFormatter<'a> {
    target_dir: &'a Path,
    transport: FileTransport,
}

impl<'a> EmlFormatter<'a> {
    pub fn new(matches: &'a ArgMatches<'a>) -> Result<Self, anyhow::Error> {
        let target_dir = match matches.value_of(arg::ARCHIVE_DIR) {
            Some(archive_dir) => Path::new(archive_dir),
            None => return Err(anyhow!("Missing value for argument '{}'", arg::ARCHIVE_DIR)),
        };

        if !target_dir.exists() {
            fs::create_dir(target_dir).context("Unable to create directory for archived emails")?;
        }

        let transport = FileTransport::new(target_dir);
        let formatter = Self {
            target_dir,
            transport,
        };

        Ok(formatter)
    }

    pub fn archive(&self, matches: &ArgMatches<'_>, email: &Email) -> Result<(), anyhow::Error> {
        if matches.is_present(arg::ARCHIVE) {
            let message_id = self
                .transport
                .send(&email.mime_format.message)
                .context("Can't save email in .eml format")?;

            let old_path = old_path(message_id.as_str(), &self.target_dir);
            let new_path = new_path(matches, message_id.as_str(), &self.target_dir);

            println!("Archiving '{}' ...", new_path.display());

            // TODO: renaming file is required because of issue/discussion https://github.com/lettre/lettre/discussions/711
            fs::rename(old_path, new_path).context("Can't rename archived email")?;
        }

        Ok(())
    }
}

fn old_path(message_id: &str, target_dir: &Path) -> PathBuf {
    let old_file_name = format!("{}.eml", message_id);
    target_dir.join(old_file_name)
}

fn new_path(matches: &ArgMatches<'_>, message_id: &str, target_dir: &Path) -> PathBuf {
    let now = std::time::SystemTime::now();
    let now_utc: chrono::DateTime<chrono::Utc> = now.into();
    let timestamp = now_utc.to_rfc3339_opts(chrono::SecondsFormat::Secs, true);

    let new_file_name = if matches.is_present(arg::DRY_RUN) {
        format!("{}_{}_dry-run.eml", timestamp, message_id)
    } else {
        format!("{}_{}.eml", timestamp, message_id)
    };

    target_dir.join(new_file_name)
}
