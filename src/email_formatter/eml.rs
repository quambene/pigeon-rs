use crate::email_builder::Email;
use anyhow::Context;
use chrono::{DateTime, Utc};
use lettre::{FileTransport, Transport};
use std::{
    fs,
    path::{Path, PathBuf},
};

/// Structure to store emails in EML format.
pub struct EmlFormatter {
    target_dir: PathBuf,
    transport: FileTransport,
}

impl EmlFormatter {
    pub fn new(target_dir: &Path) -> Result<Self, anyhow::Error> {
        if !target_dir.exists() {
            fs::create_dir(target_dir).context("Unable to create directory for archived emails")?;
        }

        let transport = FileTransport::new(target_dir);
        let formatter = Self {
            target_dir: target_dir.to_owned(),
            transport,
        };

        Ok(formatter)
    }

    pub fn archive(
        &self,
        email: &Email,
        now: DateTime<Utc>,
        dry_run: bool,
    ) -> Result<(), anyhow::Error> {
        let message_id = self
            .transport
            .send(&email.mime_format.message)
            .context("Can't save email in .eml format")?;

        let old_path = old_path(message_id.as_str(), &self.target_dir);
        let new_path = new_path(message_id.as_str(), &self.target_dir, dry_run, now);

        println!("Archiving '{}' ...", new_path.display());

        // TODO: renaming file is required because of issue/discussion https://github.com/lettre/lettre/discussions/711
        fs::rename(old_path, new_path).context("Can't rename archived email")?;

        Ok(())
    }
}

fn old_path(message_id: &str, target_dir: &Path) -> PathBuf {
    let old_file_name = format!("{}.eml", message_id);
    target_dir.join(old_file_name)
}

fn new_path(message_id: &str, target_dir: &Path, dry_run: bool, now: DateTime<Utc>) -> PathBuf {
    let timestamp = now.to_rfc3339_opts(chrono::SecondsFormat::Secs, true);

    let new_file_name = if dry_run {
        format!("{}_{}_dry-run.eml", timestamp, message_id)
    } else {
        format!("{}_{}.eml", timestamp, message_id)
    };

    target_dir.join(new_file_name)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::email_builder::{Message, MimeFormat, Receiver, Sender};
    use std::time::{Duration, SystemTime};
    use tempfile::tempdir;

    fn create_email<'a>(now: SystemTime) -> Email<'a> {
        let sender = Sender("albert@einstein.com");
        let receiver = Receiver("marie@curie.com");
        let subject = "Test subject";
        let text = "This is a test message (plaintext).";
        let html = "<p>This is a test message (html).</p>";
        let message = Message::new(subject, Some(text), Some(html));
        let mime_format = MimeFormat::new(sender, receiver, &message, None, now).unwrap();
        Email::new(sender, receiver, &message, &mime_format).unwrap()
    }

    #[test]
    fn test_archive() {
        let temp_dir = tempdir().unwrap();
        let temp_path = temp_dir.path();
        assert!(temp_path.exists(), "Missing path: {}", temp_path.display());

        let now = Utc::now();
        let now_system_time = SystemTime::UNIX_EPOCH + Duration::from_secs(now.timestamp() as u64);
        let dry_run = false;
        let email = create_email(now_system_time);

        let email_formatter = EmlFormatter::new(temp_path).unwrap();
        let res = email_formatter.archive(&email, now, dry_run);
        assert!(res.is_ok(), "{}", res.unwrap_err());

        if let Ok(entries) = fs::read_dir(temp_path) {
            let files = entries.flatten().collect::<Vec<_>>();
            assert_eq!(files.len(), 1);

            let path = &files[0].path();
            assert_eq!(path.extension().unwrap().to_str().unwrap(), "eml");

            let eml = fs::read_to_string(path).unwrap();
            assert_eq!(eml, format!("{:?}", email.mime_format));
        }
    }

    #[test]
    fn test_archive_dry() {
        let temp_dir = tempdir().unwrap();
        let temp_path = temp_dir.path();
        assert!(temp_path.exists(), "Missing path: {}", temp_path.display());

        let now = Utc::now();
        let now_system_time = SystemTime::UNIX_EPOCH + Duration::from_secs(now.timestamp() as u64);
        let dry_run = true;
        let email = create_email(now_system_time);

        let email_formatter = EmlFormatter::new(temp_path).unwrap();
        let res = email_formatter.archive(&email, now, dry_run);
        assert!(res.is_ok(), "{}", res.unwrap_err());

        if let Ok(entries) = fs::read_dir(temp_path) {
            let files = entries.flatten().collect::<Vec<_>>();
            assert_eq!(files.len(), 1);

            let path = &files[0].path();
            assert_eq!(path.extension().unwrap().to_str().unwrap(), "eml");
            assert!(path.to_str().unwrap().contains("dry-run"));

            let eml = fs::read_to_string(path).unwrap();
            assert_eq!(eml, format!("{:?}", email.mime_format));
        }
    }
}
