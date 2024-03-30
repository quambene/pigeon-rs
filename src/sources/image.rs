use crate::arg;
use anyhow::{anyhow, Context};
use clap::ArgMatches;
use polars::prelude::{DataFrame, DataType, TakeRandom};
use std::{
    fs::{self, File},
    io::Write,
    path::PathBuf,
};
use uuid::Uuid;

pub fn write_image(
    matches: &ArgMatches,
    df: DataFrame,
    file_type: &str,
) -> Result<(), anyhow::Error> {
    let target_dir = match matches.value_of(arg::SAVE_DIR) {
        Some(save_dir) => PathBuf::from(save_dir),
        None => return Err(anyhow!("Missing value for argument '{}'", arg::SAVE_DIR)),
    };

    match target_dir.exists() {
        true => (),
        false => fs::create_dir(&target_dir).context(format!(
            "Can't create directory: '{}'",
            target_dir.display()
        ))?,
    }

    let image_col = match matches.value_of(arg::IMAGE_COLUMN) {
        Some(column_name) => column_name,
        None => {
            return Err(anyhow!(
                "Missing value for argument '{}'",
                arg::IMAGE_COLUMN
            ))
        }
    };
    let name_col = match matches.value_of(arg::IMAGE_NAME) {
        Some(column_name) => column_name,
        None => return Err(anyhow!("Missing value for argument '{}'", arg::IMAGE_NAME)),
    };

    for i in 0..df.height() {
        let data_type = df
            .column(name_col)
            .context("Can't find column for image name")?
            .dtype();

        let image_name = match data_type {
            DataType::Utf8 => df
                .column(name_col)
                .context("Can't find column for image name")?
                .utf8()
                .context("Can't convert series to chunked array")?
                .get(i)
                .map(|str| str.to_string()),
            DataType::Null => None,
            _ => Some(
                df.column(name_col)
                    .context("Can't find column for image name")?
                    .get(i)?
                    .to_string(),
            ),
        };

        let image_name = match image_name {
            Some(image_name) => image_name,
            None => {
                // Indicate the missing image name by a UUID
                Uuid::new_v4().to_hyphenated().to_string()
            }
        };

        let target_file = image_name + "." + file_type;
        let target_path = target_dir.join(target_file);

        let image = df
            .column(image_col)
            .context("Can't find column for images")?
            .list()
            .context("Can't convert series to chunked array")?
            .get(i);

        println!("Save query result to file: {}", target_path.display());

        match image {
            Some(image) => {
                let bytes: Vec<u8> = image
                    .u8()
                    .context("Can't convert series to chunked array")?
                    .into_iter()
                    .map(|byte| byte.expect("Can't convert series to bytes"))
                    .collect();

                let mut file = File::create(target_path).context("Unable to create file")?;
                file.write_all(bytes.as_slice())
                    .context("Unable to write file.")?;
            }
            None => continue,
        }
    }

    Ok(())
}
