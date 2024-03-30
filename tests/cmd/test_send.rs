use assert_cmd::Command;
use predicates::str;
use std::env;

#[test]
#[ignore]
fn test_send_smtp() {
    println!("Execute 'pigeon send --connection smtp'");
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
        "--connection",
        "smtp",
    ]);
    cmd.assert().success().stdout(str::contains("abc"));
}

#[test]
#[ignore]
fn test_send_aws() {
    let sender = env::var("TEST_SENDER").expect("Missing environment variable 'TEST_SENDER'");
    let receiver = env::var("TEST_RECEIVER").expect("Missing environment variable 'TEST_RECEIVER'");

    println!("Execute 'pigeon send --connection aws ...'");
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
fn test_attachment_pdf_aws() {
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
