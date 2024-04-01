use anyhow::Context;
use polars::prelude::{DataFrame, DataType, TakeRandom};
use std::{
    fs::{self, File},
    io::Write,
    path::Path,
};
use uuid::Uuid;

pub fn write_image(
    target_dir: &Path,
    image_column: &str,
    image_name: &str,
    df: DataFrame,
    file_type: &str,
) -> Result<(), anyhow::Error> {
    match target_dir.exists() {
        true => (),
        false => fs::create_dir(&target_dir).context(format!(
            "Can't create directory: '{}'",
            target_dir.display()
        ))?,
    }

    for i in 0..df.height() {
        let data_type = df
            .column(image_name)
            .context("Can't find column for image name")?
            .dtype();

        let image_name = match data_type {
            DataType::Utf8 => df
                .column(image_name)
                .context("Can't find column for image name")?
                .utf8()
                .context("Can't convert series to chunked array")?
                .get(i)
                .map(|str| str.to_string()),
            DataType::Null => None,
            _ => Some(
                df.column(image_name)
                    .context("Can't find column for image name")?
                    .get(i)?
                    .to_string(),
            ),
        };

        let image_name = image_name.unwrap_or(Uuid::new_v4().to_hyphenated().to_string());
        let target_file = image_name + "." + file_type;
        let target_path = target_dir.join(target_file);

        let image = df
            .column(image_column)
            .context("Can't find column for images")?
            .list()
            .context("Can't convert series to chunked array")?
            .get(i);

        println!("Save query result to file: {}", target_path.display());

        if let Some(image) = image {
            let bytes = image
                .u8()
                .context("Can't convert series to chunked array")?
                .into_iter()
                .map(|byte| byte.expect("Can't convert series to bytes"))
                .collect::<Vec<_>>();

            let mut file = File::create(target_path).context("Unable to create file")?;
            file.write_all(bytes.as_slice())
                .context("Unable to write file.")?;
        }
    }

    Ok(())
}
