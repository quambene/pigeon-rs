use assert_cmd::Command;
use predicates::str;
use std::env;

#[test]
fn test_send_message_file_empty_smtp_dry() {
    println!("Execute 'pigeon send'");
    let mut cmd = Command::cargo_bin("pigeon").unwrap();
    cmd.args([
        "send",
        "albert@einstein.com",
        "marie@curie.com",
        "--message-file",
        "./test_data/empty_message.yaml",
        "--dry-run",
        "--display",
        "--assume-yes",
        "--archive",
    ]);
    cmd.assert().success().stdout(str::contains("abc"));
}

#[test]
#[ignore]
fn test_send_message_file_none_html_smtp_dry() {
    println!("Execute 'pigeon send'");
    let mut cmd = Command::cargo_bin("pigeon").unwrap();
    cmd.args([
        "send",
        "albert@einstein.com",
        "marie@curie.com",
        "--message-file",
        "./test_data/none_html_message.yaml",
        "--dry-run",
        "--display",
        "--assume-yes",
        "--archive",
    ]);
    cmd.assert().success().stdout(str::contains("abc"));
}

#[test]
fn test_send_message_file_content_none_smtp_dry() {
    println!("Execute 'pigeon send'");
    let mut cmd = Command::cargo_bin("pigeon").unwrap();
    cmd.args([
        "send",
        "albert@einstein.com",
        "marie@curie.com",
        "--message-file",
        "./test_data/content_none_message.yaml",
        "--dry-run",
        "--display",
        "--assume-yes",
        "--archive",
    ]);
    cmd.assert().success().stdout(str::contains("abc"));
}

#[test]
fn test_archive_smtp_dry() {
    println!("Execute 'pigeon send'");
    let mut cmd = Command::cargo_bin("pigeon").unwrap();
    cmd.args([
        "send",
        "albert@einstein.com",
        "marie@curie.com",
        "--message-file",
        "./test_data/message.yaml",
        "--dry-run",
        "--display",
        "--assume-yes",
        "--archive",
    ]);
    cmd.assert().success().stdout(str::contains("abc"));
}

#[test]
fn test_archive_dir_smtp_dry() {
    println!("Execute 'pigeon send'");
    let mut cmd = Command::cargo_bin("pigeon").unwrap();
    cmd.args([
        "send",
        "albert@einstein.com",
        "marie@curie.com",
        "--message-file",
        "./test_data/message.yaml",
        "--dry-run",
        "--display",
        "--assume-yes",
        "--archive",
        "--archive-dir",
        "./my-sent-emails",
    ]);
    cmd.assert().success().stdout(str::contains("abc"));
}

#[test]
fn test_send_aws_api_dry() {
    println!("Execute 'pigeon send'");
    let mut cmd = Command::cargo_bin("pigeon").unwrap();
    cmd.args([
        "send",
        "albert@einstein.com",
        "marie@curie.com",
        "--subject",
        "Test Subject",
        "--content",
        "This is a test message (plaintext).",
        "--dry-run",
        "--display",
        "--assume-yes",
        "--archive",
        "--connection",
        "aws",
    ]);
    cmd.assert().success().stdout(str::contains("abc"));
}

#[test]
#[ignore]
fn test_send_subject_content_smtp() {
    println!("Execute 'pigeon send'");
    let mut cmd = Command::cargo_bin("pigeon").unwrap();
    cmd.args([
        "send",
        "--subject",
        "Test Subject",
        "--content",
        "This is a test message (plaintext).",
        "--display",
        "--assume-yes",
        "--archive",
    ]);
    cmd.assert().success().stdout(str::contains("abc"));
}

#[test]
#[ignore]
fn test_send_message_file_smtp() {
    let sender = env::var("TEST_SENDER").expect("Missing environment variable 'TEST_SENDER'");
    let receiver = env::var("TEST_RECEIVER").expect("Missing environment variable 'TEST_RECEIVER'");

    println!("Execute 'pigeon send'");
    let mut cmd = Command::cargo_bin("pigeon").unwrap();
    cmd.args([
        "send",
        &sender,
        &receiver,
        "--message-file",
        "./test_data/message.yaml",
        "--display",
        "--assume-yes",
    ]);
    cmd.assert().success().stdout(str::contains("abc"));
}

#[test]
#[ignore]
fn test_send_message_file_aws_api() {
    let sender = env::var("TEST_SENDER").expect("Missing environment variable 'TEST_SENDER'");
    let receiver = env::var("TEST_RECEIVER").expect("Missing environment variable 'TEST_RECEIVER'");

    println!("Execute 'pigeon send'");
    let mut cmd = Command::cargo_bin("pigeon").unwrap();
    cmd.args([
        "send",
        &sender,
        &receiver,
        "--connection",
        "aws",
        "--message-file",
        "./test_data/message.yaml",
        "--display",
        "--assume-yes",
    ]);
    cmd.assert().success().stdout(str::contains("abc"));
}

#[test]
#[ignore]
fn test_attachment_pdf_smtp() {
    let sender = env::var("TEST_SENDER").expect("Missing environment variable 'TEST_SENDER'");
    let receiver = env::var("TEST_RECEIVER").expect("Missing environment variable 'TEST_RECEIVER'");

    println!("Execute 'pigeon send'");
    let mut cmd = Command::cargo_bin("pigeon").unwrap();
    cmd.args([
        "send",
        &sender,
        &receiver,
        "--message-file",
        "./test_data/message.yaml",
        "--display",
        "--assume-yes",
        "--archive",
        "--attachment",
        "./test_data/test.pdf",
    ]);
    cmd.assert().success().stdout(str::contains("abc"));
}

#[test]
#[ignore]
fn test_attachment_pdf_aws_api() {
    let sender = env::var("TEST_SENDER").expect("Missing environment variable 'TEST_SENDER'");
    let receiver = env::var("TEST_RECEIVER").expect("Missing environment variable 'TEST_RECEIVER'");

    println!("Execute 'pigeon send'");
    let mut cmd = Command::cargo_bin("pigeon").unwrap();
    cmd.args([
        "send",
        &sender,
        &receiver,
        "--connection",
        "aws",
        "--message-file",
        "./test_data/message.yaml",
        "--display",
        "--assume-yes",
        "--archive",
        "--attachment",
        "./test_data/test.pdf",
    ]);
    cmd.assert().success().stdout(str::contains("abc"));
}

#[test]
#[ignore]
fn test_send_aws_api() {
    let sender = env::var("TEST_SENDER").expect("Missing environment variable 'TEST_SENDER'");
    let receiver = env::var("TEST_RECEIVER").expect("Missing environment variable 'TEST_RECEIVER'");

    println!("Execute 'pigeon send'");
    let mut cmd = Command::cargo_bin("pigeon").unwrap();
    cmd.args([
        "send",
        &sender,
        &receiver,
        "--subject",
        "Test Subject",
        "--content",
        "This is a test message (plaintext).",
        "--display",
        "--assume-yes",
        "--archive",
        "--connection",
        "aws",
    ]);
    cmd.assert().success().stdout(str::contains("abc"));
}

#[test]
#[ignore]
fn test_send_text_file_html_file_smtp() {
    let sender = env::var("TEST_SENDER").expect("Missing environment variable 'TEST_SENDER'");
    let receiver = env::var("TEST_RECEIVER").expect("Missing environment variable 'TEST_RECEIVER'");

    println!("Execute 'pigeon send'");
    let mut cmd = Command::cargo_bin("pigeon").unwrap();
    cmd.args([
        "send",
        &sender,
        &receiver,
        "--subject",
        "Test Subject",
        "--text-file",
        "./test_data/message.txt",
        "--html-file",
        "./test_data/message.html",
        "--display",
        "--assume-yes",
        "--archive",
    ]);
    cmd.assert().success().stdout(str::contains("abc"));
}
