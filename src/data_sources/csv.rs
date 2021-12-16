use anyhow::{anyhow, Context};
use clap::ArgMatches;
use polars::prelude::{CsvReader, CsvWriter, DataFrame, SerReader, SerWriter};
use std::{fs, path::PathBuf, time::SystemTime};

use crate::arg;

pub fn read_csv(csv_file: &PathBuf) -> Result<DataFrame, anyhow::Error> {
    println!("Reading csv file '{}' ...", csv_file.display());
    let reader = CsvReader::from_path(csv_file)?.has_header(true);
    let df = reader.finish()?;
    Ok(df)
}

pub fn write_csv(matches: &ArgMatches<'_>, df: DataFrame) -> Result<(), anyhow::Error> {
    let now = SystemTime::now();
    let now_utc: chrono::DateTime<chrono::Utc> = now.into();
    let current_time = now_utc.to_rfc3339_opts(chrono::SecondsFormat::Secs, true);

    let target_dir = match matches.value_of(arg::OUTPUT_DIR) {
        Some(output_dir) => PathBuf::from(output_dir),
        None => return Err(anyhow!("Missing value for argument '{}'", arg::OUTPUT_DIR)),
    };
    let target_file = "query_".to_string() + &current_time + ".csv";

    match target_dir.exists() {
        true => (),
        false => fs::create_dir(&target_dir).context("Can't create output directory")?,
    }

    let target_path = target_dir.join(target_file);
    println!("Save query result to file: {}", target_path.display());

    let csv_file = &mut fs::File::create(target_path.to_path_buf())?;
    let timestamp_format = "%F_H:%M:%S";
    let _writer = CsvWriter::new(csv_file)
        .with_timestamp_format(timestamp_format.to_string())
        .finish(&df)?;

    Ok(())
}
