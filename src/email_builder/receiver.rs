use crate::{
    arg, cmd,
    sources::{self, ConnVars, DbConnection},
};
use anyhow::{anyhow, Context};
use clap::ArgMatches;
use polars::{
    chunked_array::{ops::TakeRandom, ChunkedArray},
    datatypes::Utf8Type,
    frame::DataFrame,
};
use std::path::Path;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Receiver<'a>(pub &'a str);

impl<'a> AsRef<str> for Receiver<'a> {
    fn as_ref(&self) -> &str {
        self.0
    }
}

#[derive(Debug, PartialEq)]
pub struct BulkReceiver {
    pub column_name: String,
    pub df_receiver: DataFrame,
}

impl BulkReceiver {
    pub fn new(column_name: String, df_receiver: DataFrame) -> Self {
        Self {
            column_name,
            df_receiver,
        }
    }

    pub fn from_args(matches: &ArgMatches) -> Result<Self, anyhow::Error> {
        let column_name = arg::value(arg::RECEIVER_COLUMN, matches)?;
        let receiver_query = matches.get_one::<String>(arg::RECEIVER_QUERY);
        let receiver_path = matches.get_one::<String>(arg::RECEIVER_FILE).map(Path::new);

        match (receiver_query, receiver_path) {
            (Some(query), None) => {
                let conn_vars = ConnVars::from_env()?;
                let ssh_tunnel = matches.get_one::<String>(arg::SSH_TUNNEL).map(|arg| arg.as_ref());
                let connection = DbConnection::new(&conn_vars, ssh_tunnel)?;
                let df_receiver = sources::query_postgres(&connection, query)?;

                if matches.get_flag(arg::DISPLAY) {
                    println!("Display query result: {}", df_receiver);
                }

                Ok(Self::new(
                    column_name.to_owned() ,
                    df_receiver,
                ))
            },
            (None, Some(path)) => {
                let df_receiver = sources::read_csv(path)?;

                if matches.get_flag(arg::DISPLAY) {
                    println!("Display csv file: {}", df_receiver);
                }

                Ok(Self::new(
                    column_name.to_owned() ,
                    df_receiver,
                ))
            },
            (Some(_), Some(_)) => {
                Err(anyhow!(
                    "Argument conflict: arguments {} and {} are not allowed at the same time. Check usage via '{} help {}'",
                    arg::RECEIVER_QUERY,
                    arg::RECEIVER_FILE,
                    cmd::BIN,
                    cmd::SEND_BULK,
                ))
            },
            (None, None) => {
                Err(anyhow!(
                    "Missing arguments: please specify argument {} or {}. Check usage via '{} help {}'",
                    arg::RECEIVER_QUERY,
                    arg::RECEIVER_FILE,
                    cmd::BIN,
                    cmd::SEND_BULK,
                ))
            },
        }
    }

    pub fn height(&self) -> usize {
        self.df_receiver.height()
    }

    pub fn receiver_column(&self) -> Result<&ChunkedArray<Utf8Type>, anyhow::Error> {
        self.column(&self.column_name)
    }

    pub fn receiver_row(&self, index: usize) -> Result<&str, anyhow::Error> {
        self.row(index, &self.column_name)
    }

    pub fn row<'a>(&'a self, index: usize, column_name: &str) -> Result<&'a str, anyhow::Error> {
        let column_value = self
            .df_receiver
            .column(column_name)
            .context(format!("Missing column '{}'", column_name))?
            .utf8()
            .context("Can't convert series to chunked array")?
            .get(index);

        match column_value {
            Some(column_value) => Ok(column_value),
            None => Err(anyhow!(
                "Missing value for column '{}' in row {}",
                column_name,
                index
            )),
        }
    }

    pub fn column<'a>(
        &'a self,
        column_name: &str,
    ) -> Result<&'a ChunkedArray<Utf8Type>, anyhow::Error> {
        let column = self
            .df_receiver
            .column(column_name)
            .context(format!("Missing column '{column_name}'"))?
            .utf8()
            .context("Can't convert series to chunked array")?;

        Ok(column)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::app;
    use polars::{prelude::NamedFrom, series::Series};

    #[test]
    fn test_bulk_receiver_from_args_receiver_file() {
        let args = vec![
            "pigeon",
            "send-bulk",
            "albert@einstein.com",
            "--receiver-file",
            "./test_data/receiver.csv",
            "--message-file",
            "./test_data/message.yaml",
        ];
        let app = app();
        let matches = app.get_matches_from(args);
        let subcommand_matches = matches.subcommand_matches("send-bulk").unwrap();

        let res = BulkReceiver::from_args(subcommand_matches);
        assert!(res.is_ok(), "{}", res.unwrap_err());

        let actual = res.unwrap();
        let first_name_column = Series::new("first_name", &["Marie", "Alexandre"]);
        let last_name_column = Series::new("last_name", &["Curie", "Grothendieck"]);
        let email_column = Series::new("email", &["marie@curie.com", "alexandre@grothendieck.com"]);
        let expected =
            DataFrame::new(vec![first_name_column, last_name_column, email_column]).unwrap();
        assert_eq!(
            actual,
            BulkReceiver {
                column_name: "email".to_owned(),
                df_receiver: expected
            }
        );
    }

    #[test]
    fn test_bulk_receiver_from_args_receiver_column() {
        let args = vec![
            "pigeon",
            "send-bulk",
            "albert@einstein.com",
            "--receiver-file",
            "./test_data/contacts.csv",
            "--message-file",
            "./test_data/message.yaml",
            "--receiver-column",
            "contact",
        ];
        let app = app();
        let matches = app.get_matches_from(args);
        let subcommand_matches = matches.subcommand_matches("send-bulk").unwrap();

        let res = BulkReceiver::from_args(subcommand_matches);
        assert!(res.is_ok(), "{}", res.unwrap_err());

        let actual = res.unwrap();
        let first_name_column = Series::new("first_name", &["Marie", "Alexandre"]);
        let last_name_column = Series::new("last_name", &["Curie", "Grothendieck"]);
        let email_column = Series::new(
            "contact",
            &["marie@curie.com", "alexandre@grothendieck.com"],
        );
        let expected =
            DataFrame::new(vec![first_name_column, last_name_column, email_column]).unwrap();
        assert_eq!(
            actual,
            BulkReceiver {
                column_name: "contact".to_owned(),
                df_receiver: expected
            }
        );
    }
}
