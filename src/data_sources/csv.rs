use anyhow::Context;
use polars::prelude::{CsvReader, CsvWriter, DataFrame, SerReader, SerWriter};
use std::{fs, path::Path, time::SystemTime};

pub fn read_csv(csv_file: &Path) -> Result<DataFrame, anyhow::Error> {
    println!("Reading csv file '{}' ...", csv_file.display());
    let reader = CsvReader::from_path(csv_file)?.has_header(true);
    let df = reader.finish()?;
    Ok(df)
}

pub fn write_csv(save_dir: &Path, mut df: DataFrame) -> Result<(), anyhow::Error> {
    let now = SystemTime::now();
    let now_utc: chrono::DateTime<chrono::Utc> = now.into();
    let current_time = now_utc.to_rfc3339_opts(chrono::SecondsFormat::Secs, true);

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
        .finish(&mut df)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use polars::{datatypes::AnyValue, frame::row::Row};

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
}
