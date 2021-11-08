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

pub fn check_send_status(res: Result<(), anyhow::Error>) -> String {
    let status: String;
    match res {
        Ok(_) => status = format_green("ok"),
        Err(err) => status = format!("{}: {}", format_red("failed"), err),
    }
    status
}