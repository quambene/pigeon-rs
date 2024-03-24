use assert_cmd::Command;
use predicates::{boolean::PredicateBooleanExt, str};

#[test]
#[ignore]
fn test_send_bulk_subject_content_dry() {
    println!("Execute 'pigeon send-bulk'");
    let mut cmd = Command::cargo_bin("pigeon").unwrap();
    cmd.args([
        "send-bulk",
        "albert@einstein.com",
        "--receiver-file",
        "./test_data/receiver.csv",
        "--subject",
        "Test Subject",
        "--content",
        "This is a test message (plaintext).",
        "--dry-run",
        "--display",
        "--assume-yes",
    ]);
    cmd.assert().success().stdout(
        str::contains("Reading csv file './test_data/receiver.csv' ...").and(
            str::contains("Display csv file:")
                .and(str::contains("Display emails:"))
                .and(str::contains("Dry run: \u{1b}[32mactivated\u{1b}[0m")),
        ),
    );
}

#[test]
#[ignore]
fn test_send_bulk_text_file_html_file_dry() {
    println!("Execute 'pigeon send-bulk'");
    let mut cmd = Command::cargo_bin("pigeon").unwrap();
    cmd.args([
        "send-bulk",
        "albert@einstein.com",
        "--receiver-file",
        "./test_data/receiver.csv",
        "--subject",
        "Test Subject",
        "--text-file",
        "./test_data/message.txt",
        "--html-file",
        "./test_data/message.html",
        "--dry-run",
        "--display",
        "--assume-yes",
    ]);
    cmd.assert().success();
}

#[test]
#[ignore]
fn test_send_bulk_message_file_dry() {
    println!("Execute 'pigeon send-bulk'");
    let mut cmd = Command::cargo_bin("pigeon").unwrap();
    cmd.args([
        "send-bulk",
        "albert@einstein.com",
        "--receiver-file",
        "./test_data/receiver.csv",
        "--message-file",
        "./test_data/message.yaml",
        "--dry-run",
        "--display",
        "--assume-yes",
    ]);
    cmd.assert().success();
}

#[test]
#[ignore]
fn test_send_bulk_receiver_column_dry() {
    println!("Execute 'pigeon send-bulk'");
    let mut cmd = Command::cargo_bin("pigeon").unwrap();
    cmd.args([
        "send-bulk",
        "albert@einstein.com",
        "--receiver-file",
        "./test_data/contacts.csv",
        "--message-file",
        "./test_data/message.yaml",
        "--receiver-column",
        "contact",
        "--dry-run",
        "--display",
        "--assume-yes",
    ]);
    cmd.assert().success();
}

#[test]
#[ignore]
fn test_send_bulk_personalize_dry() {
    println!("Execute 'pigeon send-bulk'");
    let mut cmd = Command::cargo_bin("pigeon").unwrap();
    cmd.args([
        "send-bulk",
        "albert@einstein.com",
        "--receiver-file",
        "./test_data/receiver.csv",
        "--message-file",
        "./test_data/message_personalized.yaml",
        "--personalize",
        "first_name",
        "last_name",
        "--dry-run",
        "--display",
        "--assume-yes",
    ]);
    cmd.assert().success();
}

#[test]
#[ignore]
fn test_archive_dry() {
    println!("Execute 'pigeon send-bulk'");
    let mut cmd = Command::cargo_bin("pigeon").unwrap();
    cmd.args([
        "send-bulk",
        "albert@einstein.com",
        "--receiver-file",
        "./test_data/receiver.csv",
        "--message-file",
        "./test_data/message.yaml",
        "--dry-run",
        "--display",
        "--assume-yes",
        "--archive",
    ]);
    cmd.assert().success();
}

#[test]
#[ignore]
fn test_archive_dir_dry() {
    println!("Execute 'pigeon send-bulk'");
    let mut cmd = Command::cargo_bin("pigeon").unwrap();
    cmd.args([
        "send-bulk",
        "albert@einstein.com",
        "--receiver-file",
        "./test_data/receiver.csv",
        "--message-file",
        "./test_data/message.yaml",
        "--dry-run",
        "--display",
        "--assume-yes",
        "--archive",
        "--archive-dir",
        "./my-sent-emails",
    ]);
    cmd.assert().success();
}

#[test]
#[ignore]
fn test_attachment_pdf_dry() {
    println!("Execute 'pigeon send-bulk'");
    let mut cmd = Command::cargo_bin("pigeon").unwrap();
    cmd.args([
        "send-bulk",
        "albert@einstein.com",
        "--receiver-file",
        "./test_data/receiver.csv",
        "--message-file",
        "./test_data/message.yaml",
        "--dry-run",
        "--display",
        "--assume-yes",
        "--archive",
        "--attachment",
        "./test_data/test.pdf",
    ]);
    cmd.assert().success();
}

#[test]
#[ignore]
fn test_attachment_png_dry() {
    println!("Execute 'pigeon send-bulk'");
    let mut cmd = Command::cargo_bin("pigeon").unwrap();
    cmd.args([
        "send-bulk",
        "albert@einstein.com",
        "--receiver-file",
        "./test_data/receiver.csv",
        "--message-file",
        "./test_data/message.yaml",
        "--dry-run",
        "--display",
        "--assume-yes",
        "--archive",
        "--attachment",
        "./test_data/test.png",
    ]);
    cmd.assert().success();
}

#[test]
#[ignore]
fn test_attachment_odt_dry() {
    println!("Execute 'pigeon send-bulk'");
    let mut cmd = Command::cargo_bin("pigeon").unwrap();
    cmd.args([
        "send-bulk",
        "albert@einstein.com",
        "--receiver-file",
        "./test_data/receiver.csv",
        "--message-file",
        "./test_data/message.yaml",
        "--dry-run",
        "--display",
        "--assume-yes",
        "--archive",
        "--attachment",
        "./test_data/test.odt",
    ]);
    cmd.assert().success();
}

#[test]
#[ignore]
fn test_send_bulk_aws_dry() {
    println!("Execute 'pigeon send-bulk'");
    let mut cmd = Command::cargo_bin("pigeon").unwrap();
    cmd.args([
        "send-bulk",
        "albert@einstein.com",
        "--receiver-file",
        "./test_data/receiver.csv",
        "--message-file",
        "./test_data/message.yaml",
        "--dry-run",
        "--display",
        "--assume-yes",
        "--connection",
    ]);
    cmd.assert().success();
}
