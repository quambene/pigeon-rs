use std::{fs, path::Path};

pub fn format_green(text: &str) -> String {
    const GREEN: &str = "\x1b[32m";
    const END: &str = "\x1b[0m";
    let green_text = format!("{}{}{}", GREEN, text, END);
    green_text
}

pub fn format_red(text: &str) -> String {
    const RED: &str = "\x1b[31m";
    const END: &str = "\x1b[0m";
    let red_text = format!("{}{}{}", RED, text, END);
    red_text
}

pub fn read_file(path: &Path) -> Result<String, anyhow::Error> {
    println!("Reading file '{}' ...", path.display());
    let content = fs::read_to_string(path)?;
    Ok(content)
}
