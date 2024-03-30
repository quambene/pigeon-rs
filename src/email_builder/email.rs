use super::{BulkReceiver, Receiver, Sender};
use super::{Receiver, Sender};
use crate::email_builder::{Email, Message, Message, MimeFormat, MimeFormat};
use anyhow::Result;
use std::{path::Path, time::SystemTime};

#[derive(Debug)]
pub struct Email<'a> {
    pub sender: Sender<'a>,
    pub receiver: Receiver<'a>,
    pub message: Message,
    pub mime_format: MimeFormat,
}

impl<'a> Email<'a> {
    pub fn new(
        sender: Sender<'a>,
        receiver: Receiver<'a>,
        message: &Message,
        mime_format: &MimeFormat,
    ) -> Result<Self, anyhow::Error> {
        let email = Email {
            sender,
            receiver,
            message: message.to_owned(),
            mime_format: mime_format.to_owned(),
        };
        Ok(email)
    }
}

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
        personalized_columns: &[&str],
    ) -> Result<Self, anyhow::Error> {
        let now = SystemTime::now();
        let mut emails: Vec<Email> = vec![];

        if personalized_columns.is_empty() {
            let receivers = bulk_receiver.receiver_column()?;
            for receiver in receivers.into_iter().flatten() {
                let mime_format =
                    MimeFormat::new(sender, Receiver(receiver), message, attachment, now)?;
                let email = Email::new(sender, Receiver(receiver), message, &mime_format)?;
                emails.push(email);
            }
        } else {
            for i in 0..bulk_receiver.height() {
                let mut message = message.clone();

                for &col_name in personalized_columns.iter() {
                    let col_value = bulk_receiver.row(i, col_name)?;
                    message.personalize(col_name, col_value);
                }

                let receiver = bulk_receiver.receiver_row(i)?;
                let mime_format =
                    MimeFormat::new(sender, Receiver(receiver), &message, attachment, now)?;
                let email = Email::new(sender, Receiver(receiver), &message, &mime_format)?;

                emails.push(email);
            }
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

        let res = BulkEmail::new(sender, &receivers, &message, None, &[]);
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

    #[test]
    fn test_bulk_email_personalized() {
        let sender = Sender("albert@einstein.com");
        let subject = "Test Subject";
        let text = r#"Dear {first_name} {last_name},
This is a test message (plaintext)."#;
        let html = r#"Dear {first_name} {last_name},
<br>
<br>
This is a test message (html)."#;
        let message = Message::new(subject, Some(text), Some(html));
        let first_name_column = Series::new("first_name", &["Marie", "Emmy"]);
        let last_name_column = Series::new("last_name", &["Curie", "Noether"]);
        let email_column = Series::new("email", &["marie@curie.com", "emmy@noether.com"]);
        let df_receiver =
            DataFrame::new(vec![first_name_column, last_name_column, email_column]).unwrap();
        let receivers = BulkReceiver::new("email".to_owned(), df_receiver);

        let res = BulkEmail::new(
            sender,
            &receivers,
            &message,
            None,
            &["first_name", "last_name"],
        );
        assert!(res.is_ok());

        let emails = res.unwrap().emails;
        let receivers = emails
            .iter()
            .map(|email| email.receiver)
            .collect::<Vec<_>>();
        assert!(receivers.contains(&Receiver("marie@curie.com")));
        assert!(receivers.contains(&Receiver("emmy@noether.com")));

        let text_messages = emails
            .iter()
            .map(|email| email.message.text.as_ref().unwrap().as_str())
            .collect::<Vec<_>>();
        assert!(text_messages.contains(&"Dear Marie Curie,\nThis is a test message (plaintext)."));
        assert!(text_messages.contains(&"Dear Emmy Noether,\nThis is a test message (plaintext)."));

        let html_messages = emails
            .iter()
            .map(|email| email.message.html.as_ref().unwrap().as_str())
            .collect::<Vec<_>>();
        assert!(html_messages
            .contains(&"Dear Marie Curie,\n<br>\n<br>\nThis is a test message (html)."));
        assert!(html_messages
            .contains(&"Dear Emmy Noether,\n<br>\n<br>\nThis is a test message (html)."));
    }
}
