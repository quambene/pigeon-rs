pub fn format_green(text: &str) -> String {
    const GREEN: &str = "\x1b[32m";
    const END: &str = "\x1b[0m";
    let green_text = format!("{}{}{}", GREEN, text, END);
    green_text
}

#[allow(dead_code)]
pub fn format_red(text: &str) -> String {
    const RED: &str = "\x1b[31m";
    const END: &str = "\x1b[0m";
    let red_text = format!("{}{}{}", RED, text, END);
    red_text
}

#[allow(dead_code)]
pub fn check_send_status(res: Result<String, anyhow::Error>) -> String {
    let status: String;
    match res {
        Ok(ok) => status = format_green(&ok),
        Err(err) => status = format!("{}: {}", format_red("FAILED"), err),
    }
    status
}
