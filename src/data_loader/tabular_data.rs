use anyhow::{anyhow, Context};
use polars::{
    chunked_array::ChunkedArray,
    prelude::{DataFrame, TakeRandom, Utf8Type},
};

pub struct TabularData;

impl TabularData {
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
