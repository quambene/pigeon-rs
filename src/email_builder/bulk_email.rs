use super::{Receiver, Sender};
use crate::{
    data_loader::TabularData,
    email_builder::{Email, Message, MimeFormat},
};
use anyhow::Result;
use clap::Values;
use polars::prelude::DataFrame;
use std::{path::Path, time::SystemTime};

#[derive(Debug)]
pub struct BulkEmail<'a> {
    pub emails: Vec<Email<'a>>,
}

impl<'a> BulkEmail<'a> {
    pub fn new(
        sender: Sender<'a>,
        receiver_column_name: &str,
        df_receiver: &'a DataFrame,
        message: &'a Message,
        attachment: Option<&Path>,
    ) -> Result<Self, anyhow::Error> {
        let now = SystemTime::now();
        let mut emails: Vec<Email> = vec![];
        let receivers = TabularData::column(receiver_column_name, df_receiver)?;

        for receiver in receivers {
            match receiver {
                Some(receiver) => {
                    let mime_format =
                        MimeFormat::new(sender, Receiver(receiver), message, attachment, now)?;
                    let email = Email::new(sender, Receiver(receiver), message, &mime_format)?;
                    emails.push(email);
                }
                None => continue,
            }
        }

        Ok(BulkEmail { emails })
    }

    pub fn personalize(
        sender: Sender<'a>,
        receiver_column_name: &str,
        df_receiver: &'a DataFrame,
        default_message: &Message,
        personalized_columns: Values,
        attachment: Option<&Path>,
    ) -> Result<Self, anyhow::Error> {
        let now = SystemTime::now();
        let mut emails: Vec<Email> = vec![];
        let columns: Vec<&str> = personalized_columns.collect();

        for i in 0..df_receiver.height() {
            let mut message = default_message.clone();
            message.personalize(i, df_receiver, &columns)?;

            let receiver = TabularData::row(i, receiver_column_name, df_receiver)?;
            let mime_format =
                MimeFormat::new(sender, Receiver(receiver), &message, attachment, now)?;
            let email = Email::new(sender, Receiver(receiver), &message, &mime_format)?;

            emails.push(email);
        }

        Ok(BulkEmail { emails })
    }
}
