use super::{BulkReceiver, Receiver, Sender};
use crate::email_builder::{Email, Message, MimeFormat};
use anyhow::Result;
use std::{path::Path, time::SystemTime};

#[derive(Debug)]
pub struct BulkEmail<'a> {
    pub emails: Vec<Email<'a>>,
}

impl<'a> BulkEmail<'a> {
    pub fn new(
        sender: Sender<'a>,
        bulk_receiver: &'a BulkReceiver,
        message: &'a Message,
        attachment: Option<&Path>,
    ) -> Result<Self, anyhow::Error> {
        let now = SystemTime::now();
        let mut emails: Vec<Email> = vec![];
        let receivers = bulk_receiver.receiver_column()?;

        for receiver in receivers {
            if let Some(receiver) = receiver {
                let mime_format =
                    MimeFormat::new(sender, Receiver(receiver), message, attachment, now)?;
                let email = Email::new(sender, Receiver(receiver), message, &mime_format)?;
                emails.push(email);
            }
        }

        Ok(BulkEmail { emails })
    }

    pub fn personalize(
        sender: Sender<'a>,
        receivers: &'a BulkReceiver,
        message: &Message,
        columns: &[&str],
        attachment: Option<&Path>,
    ) -> Result<Self, anyhow::Error> {
        let now = SystemTime::now();
        let mut emails: Vec<Email> = vec![];

        for i in 0..receivers.height() {
            let mut message = message.clone();

            for &col_name in columns.iter() {
                let col_value = receivers.row(i, col_name)?;
                message.personalize(col_name, col_value);
            }

            let receiver = receivers.receiver_row(i)?;
            let mime_format =
                MimeFormat::new(sender, Receiver(receiver), &message, attachment, now)?;
            let email = Email::new(sender, Receiver(receiver), &message, &mime_format)?;

            emails.push(email);
        }

        Ok(BulkEmail { emails })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use polars::{frame::DataFrame, prelude::NamedFrom, series::Series};

    #[test]
    fn test_bulk_email() {
        let sender = Sender("albert@einstein.com");
        let subject = "Test Subject";
        let text = "This is a test message (plaintext).";
        let html = "<p>This is a test message (html).</p>";
        let message = Message::new(subject, Some(text), Some(html));
        let column_name = "email";
        let receiver_column = Series::new(column_name, &["marie@curie.com", "emmy@noether.com"]);
        let df_receiver = DataFrame::new(vec![receiver_column]).unwrap();
        let receivers = BulkReceiver::new(column_name.to_owned(), df_receiver);

        let res = BulkEmail::new(sender, &receivers, &message, None);
        assert!(res.is_ok());

        let emails = res.unwrap().emails;
        assert_eq!(emails.len(), 2);
        assert!(emails.iter().any(|email| email.sender == sender));

        let receivers = emails
            .iter()
            .map(|email| email.receiver)
            .collect::<Vec<_>>();
        assert!(receivers.contains(&Receiver("marie@curie.com")));
        assert!(receivers.contains(&Receiver("emmy@noether.com")));
    }
}
