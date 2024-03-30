use crate::{
    arg, cmd,
    data_sources::{self, ConnVars, DbConnection},
};
use anyhow::{anyhow, Context};
use clap::ArgMatches;
use polars::{
    chunked_array::{ops::TakeRandom, ChunkedArray},
    datatypes::Utf8Type,
    frame::DataFrame,
};
use std::path::Path;

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
        let receiver_query = matches.value_of(arg::RECEIVER_QUERY);
        let receiver_path = matches.value_of(arg::RECEIVER_FILE).map(Path::new);

        match (receiver_query, receiver_path) {
            (Some(query), None) => {
                let conn_vars = ConnVars::from_env()?;
                let ssh_tunnel = matches.value_of(arg::SSH_TUNNEL);
                let connection = DbConnection::new(&conn_vars, ssh_tunnel)?;
                let df_receiver = data_sources::query_postgres(&connection, query)?;

                if matches.is_present(arg::DISPLAY) {
                    println!("Display query result: {}", df_receiver);
                }

                Ok(Self::new(
                    column_name.to_owned() ,
                    df_receiver,
                ))
            },
            (None, Some(path)) => {
                let df_receiver = data_sources::read_csv(&path)?;

                if matches.is_present(arg::DISPLAY) {
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

    pub fn receiver_column<'a>(&'a self) -> Result<&'a ChunkedArray<Utf8Type>, anyhow::Error> {
        self.column(&self.column_name)
    }

    pub fn receiver_row<'a>(&'a self, index: usize) -> Result<&'a str, anyhow::Error> {
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
