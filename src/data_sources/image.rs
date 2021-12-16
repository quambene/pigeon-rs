use anyhow::Context;
use polars::prelude::DataFrame;
use std::{
    fs::{self, File},
    io::Write,
    path::PathBuf,
    time::SystemTime,
};

pub fn write_image(df: DataFrame, file_type: &str) -> Result<(), anyhow::Error> {
    let target_dir = &PathBuf::from("./output");

    match target_dir.exists() {
        true => (),
        false => fs::create_dir(target_dir).context("Can't create output directory")?,
    }

    let chunked_array = df[0]
        .list()
        .context("Can't convert series to chunked array")?;

    for image_opt in chunked_array {
        match image_opt {
            Some(image) => {
                let now = SystemTime::now();
                let now_utc: chrono::DateTime<chrono::Utc> = now.into();
                let current_time = now_utc.to_rfc3339_opts(chrono::SecondsFormat::Nanos, true);
                let target_file = "query_".to_string() + &current_time + "." + file_type;
                let target_path = target_dir.join(target_file);
                println!("Save query result to file: {}", target_path.display());

                let bytes: Vec<u8> = image
                    .u8()
                    .context("Can't convert series to chunked array")?
                    .into_iter()
                    .map(|el| el.expect("Can't convert series to bytes"))
                    .collect();

                let mut file = File::create(target_path).expect("Unable to create file");
                file.write_all(bytes.as_slice())
                    .expect("Unable to write file.");
            }
            None => continue,
        }
    }

    Ok(())
}
