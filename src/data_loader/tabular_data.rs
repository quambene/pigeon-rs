use std::path::PathBuf;

use anyhow::{anyhow, Context};
use clap::ArgMatches;
use polars::{
    chunked_array::ChunkedArray,
    prelude::{DataFrame, TakeRandom, Utf8Type},
};

use crate::{
    arg,
    data_sources::{query_postgres, read_csv},
    email_builder::Receiver,
};

pub struct TabularData;

impl TabularData {
    pub fn from_query(matches: &ArgMatches) -> Result<DataFrame, anyhow::Error> {
        let receiver_query = Receiver::query(matches)?;
        let df_receiver = query_postgres(matches, receiver_query)?;

        if matches.is_present(arg::DISPLAY) {
            println!("Display query result: {}", df_receiver);
        }

        Ok(df_receiver)
    }

    pub fn from_file(matches: &ArgMatches) -> Result<DataFrame, anyhow::Error> {
        let receiver_file = Receiver::file_name(matches)?;
        let path = PathBuf::from(receiver_file);
        let df_receiver = read_csv(&path)?;

        if matches.is_present(arg::DISPLAY) {
            println!("Display csv file: {}", df_receiver);
        }

        Ok(df_receiver)
    }

    pub fn row<'a>(
        index: usize,
        column_name: &str,
        df: &'a DataFrame,
    ) -> Result<&'a str, anyhow::Error> {
        let column_value = df
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
        column_name: &str,
        df: &'a DataFrame,
    ) -> Result<&'a ChunkedArray<Utf8Type>, anyhow::Error> {
        let column = df
            .column(column_name)
            .context(format!("Missing column '{}'", column_name))?
            .utf8()
            .context("Can't convert series to chunked array")?;

        Ok(column)
    }
}
