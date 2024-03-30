use anyhow::Context;
use chrono::{DateTime, Utc};
use polars::prelude::{CsvReader, CsvWriter, DataFrame, SerReader, SerWriter};
use std::{fs, path::Path};

pub fn read_csv(csv_file: &Path) -> Result<DataFrame, anyhow::Error> {
    println!("Reading csv file '{}' ...", csv_file.display());
    let reader = CsvReader::from_path(csv_file)?.has_header(true);
    let df = reader.finish()?;
    Ok(df)
}

pub fn write_csv(
    df: &mut DataFrame,
    save_dir: &Path,
    now: DateTime<Utc>,
) -> Result<(), anyhow::Error> {
    let current_time = now.to_rfc3339_opts(chrono::SecondsFormat::Secs, true);

    let target_file = format!("query_{}.csv", &current_time);

    match save_dir.exists() {
        true => (),
        false => fs::create_dir(save_dir)
            .context(format!("Can't create directory: '{}'", save_dir.display()))?,
    }

    let target_path = save_dir.join(target_file);
    println!("Save query result to file: {}", target_path.display());

    let csv_file = &mut fs::File::create(target_path)?;
    let timestamp_format = "%F_H:%M:%S";

    CsvWriter::new(csv_file)
        .with_datetime_format(Some(timestamp_format.to_string()))
        .finish(df)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use polars::{datatypes::AnyValue, frame::row::Row, prelude::NamedFrom, series::Series};
    use tempfile::tempdir;

    #[test]
    fn test_read_csv() {
        let csv_file = Path::new("./test_data/receiver.csv");
        let res = read_csv(csv_file);
        assert!(res.is_ok());

        let df_receiver = res.unwrap();
        assert_eq!(
            df_receiver.get_column_names(),
            &["first_name", "last_name", "email"]
        );
        assert_eq!(
            df_receiver.get_row(0).unwrap(),
            Row::new(
                ["Marie", "Curie", "marie@curie.com"]
                    .into_iter()
                    .map(|s| AnyValue::Utf8(s))
                    .collect()
            )
        );
        assert_eq!(
            df_receiver.get_row(1).unwrap(),
            Row::new(
                ["Alexandre", "Grothendieck", "alexandre@grothendieck.com"]
                    .into_iter()
                    .map(|s| AnyValue::Utf8(s))
                    .collect()
            )
        );
    }

    #[test]
    fn test_write_csv() {
        let timestamp = chrono::DateTime::parse_from_rfc3339("2024-01-01T14:00:00Z").unwrap();
        let temp_dir = tempdir().unwrap();
        let temp_path = temp_dir.path();
        assert!(temp_path.exists(), "Missing path: {}", temp_path.display());

        let first_name_column = Series::new("first_name", &["Marie", "Alexandre"]);
        let last_name_column = Series::new("last_name", &["Curie", "Grothendieck"]);
        let email_column = Series::new("email", &["marie@curie.com", "alexandre@grothendieck.com"]);
        let mut df_receiver =
            DataFrame::new(vec![first_name_column, last_name_column, email_column]).unwrap();

        let res = write_csv(&mut df_receiver, temp_path, timestamp.naive_utc().and_utc());
        assert!(res.is_ok(), "{}", res.unwrap_err());

        let csv_file = temp_path.join(format!(
            "query_{}.csv",
            timestamp.to_rfc3339_opts(chrono::SecondsFormat::Secs, true),
        ));
        assert!(csv_file.exists());

        let df_receiver = read_csv(&csv_file).unwrap();
        assert_eq!(
            df_receiver.get_column_names(),
            &["first_name", "last_name", "email"]
        );
        assert_eq!(
            df_receiver.get_row(0).unwrap(),
            Row::new(
                ["Marie", "Curie", "marie@curie.com"]
                    .into_iter()
                    .map(|s| AnyValue::Utf8(s))
                    .collect()
            )
        );
        assert_eq!(
            df_receiver.get_row(1).unwrap(),
            Row::new(
                ["Alexandre", "Grothendieck", "alexandre@grothendieck.com"]
                    .into_iter()
                    .map(|s| AnyValue::Utf8(s))
                    .collect()
            )
        );
    }
}
