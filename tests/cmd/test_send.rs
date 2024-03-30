use assert_cmd::Command;
use predicates::{boolean::PredicateBooleanExt, str};
use std::env;
use tempfile::tempdir;

/// This test requires the following environment variables:
///     - SMTP_SERVER
///     - SMTP_USERNAME
///     - SMTP_PASSWORD
///     - TEST_SENDER
///     - TEST_RECEIVER
#[test]
#[ignore]
fn test_send_smtp() {
    let temp_dir = tempdir().unwrap();
    let temp_path = temp_dir.path();
    assert!(temp_path.exists(), "Missing path: {}", temp_path.display());

    let sender = env::var("TEST_SENDER").expect("Missing environment variable 'TEST_SENDER'");
    let receiver = env::var("TEST_RECEIVER").expect("Missing environment variable 'TEST_RECEIVER'");

    println!("Execute 'pigeon send --connection smtp'");
    let mut cmd = Command::cargo_bin("pigeon").unwrap();
    cmd.current_dir(temp_path);
    cmd.args([
        "send",
        &sender,
        &receiver,
        "--message-file",
        "./test_data/message.yaml",
        "--attachment",
        "./test_data/test.pdf",
        "--archive",
        "--archive-dir",
        "./my-sent-emails",
        "--display",
        "--assume-yes",
        "--connection",
        "smtp",
    ]);
    cmd.assert().success().stdout(
        str::contains("Reading csv file './test_data/receiver.csv' ...")
            .and(str::contains("Display csv file:").and(str::contains("Display emails:"))),
    );

    assert!(temp_path.join("my-sent-emails").exists());
}

/// This test requires the following environment variables:
///     - AWS_ACCESS_KEY_ID
///     - AWS_SECRET_ACCESS_KEY
///     - AWS_REGION
///     - TEST_SENDER
///     - TEST_RECEIVER
#[test]
#[ignore]
fn test_send_aws() {
    let temp_dir = tempdir().unwrap();
    let temp_path = temp_dir.path();
    assert!(temp_path.exists(), "Missing path: {}", temp_path.display());

    let sender = env::var("TEST_SENDER").expect("Missing environment variable 'TEST_SENDER'");
    let receiver = env::var("TEST_RECEIVER").expect("Missing environment variable 'TEST_RECEIVER'");

    println!("Execute 'pigeon send --connection aws ...'");
    let mut cmd = Command::cargo_bin("pigeon").unwrap();
    cmd.current_dir(temp_path);
    cmd.args([
        "send",
        &sender,
        &receiver,
        "--message-file",
        "./test_data/message.yaml",
        "--attachment",
        "./test_data/test.pdf",
        "--archive",
        "--archive-dir",
        "./my-sent-emails",
        "--display",
        "--assume-yes",
        "--archive",
        "--connection",
        "aws",
    ]);
    cmd.assert().success().stdout(
        str::contains("Reading csv file './test_data/receiver.csv' ...")
            .and(str::contains("Display csv file:").and(str::contains("Display emails:"))),
    );

    assert!(temp_path.join("my-sent-emails").exists());
}

#[test]
fn test_send_smtp_dry() {
    let temp_dir = tempdir().unwrap();
    let temp_path = temp_dir.path();
    assert!(temp_path.exists(), "Missing path: {}", temp_path.display());

    let sender = env::var("TEST_SENDER").expect("Missing environment variable 'TEST_SENDER'");
    let receiver = env::var("TEST_RECEIVER").expect("Missing environment variable 'TEST_RECEIVER'");

    println!("Execute 'pigeon send --connection smtp'");
    let mut cmd = Command::cargo_bin("pigeon").unwrap();
    cmd.current_dir(temp_path);
    cmd.args([
        "send",
        &sender,
        &receiver,
        "--message-file",
        "./test_data/message.yaml",
        "--attachment",
        "./test_data/test.pdf",
        "--archive",
        "--archive-dir",
        "./my-sent-emails",
        "--display",
        "--assume-yes",
        "--connection",
        "smtp",
        "--dry-run",
    ]);
    cmd.assert().success().stdout(
        str::contains("Reading csv file './test_data/receiver.csv' ...")
            .and(str::contains("Display csv file:").and(str::contains("Display emails:"))),
    );

    assert!(temp_path.join("my-sent-emails").exists());
}

#[test]
fn test_send_aws_dry() {
    let temp_dir = tempdir().unwrap();
    let temp_path = temp_dir.path();
    assert!(temp_path.exists(), "Missing path: {}", temp_path.display());

    let sender = env::var("TEST_SENDER").expect("Missing environment variable 'TEST_SENDER'");
    let receiver = env::var("TEST_RECEIVER").expect("Missing environment variable 'TEST_RECEIVER'");

    println!("Execute 'pigeon send --connection aws ...'");
    let mut cmd = Command::cargo_bin("pigeon").unwrap();
    cmd.current_dir(temp_path);
    cmd.args([
        "send",
        &sender,
        &receiver,
        "--message-file",
        "./test_data/message.yaml",
        "--attachment",
        "./test_data/test.pdf",
        "--archive",
        "--archive-dir",
        "./my-sent-emails",
        "--display",
        "--assume-yes",
        "--archive",
        "--connection",
        "aws",
        "dry-run",
    ]);
    cmd.assert().success().stdout(
        str::contains("Reading csv file './test_data/receiver.csv' ...")
            .and(str::contains("Display csv file:").and(str::contains("Display emails:"))),
    );

    assert!(temp_path.join("my-sent-emails").exists());
}
