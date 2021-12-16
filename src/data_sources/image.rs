use crate::arg;
use anyhow::{anyhow, Context};
use clap::ArgMatches;
use polars::prelude::{DataFrame, TakeRandom};
use std::{
    fs::{self, File},
    io::Write,
    path::PathBuf,
};

pub fn write_image(
    matches: &ArgMatches<'_>,
    df: DataFrame,
    file_type: &str,
) -> Result<(), anyhow::Error> {
    let target_dir = &PathBuf::from("./output");

    match target_dir.exists() {
        true => (),
        false => fs::create_dir(target_dir).context("Can't create output directory")?,
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
        let image_name = df
            .column(name_col)
            .context("Can't find column for image name")?
            .get(i)
            .to_string();

        let target_file = image_name + "." + file_type;
        let target_path = target_dir.join(target_file);

        let image_opt = df
            .column(image_col)
            .context("Can't find column for images")?
            .list()
            .context("Can't convert series to chunked array")?
            .get(i);

        println!("Save query result to file: {}", target_path.display());

        match image_opt {
            Some(image) => {
                let bytes: Vec<u8> = image
                    .u8()
                    .context("Can't convert series to chunked array")?
                    .into_iter()
                    .map(|el| el.expect("Can't convert series to bytes"))
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
