use assert_cmd::Command;
use predicates::{boolean::PredicateBooleanExt, str};
use std::{env, fs};
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
    let sender = env::var("TEST_SENDER").expect("Missing environment variable 'TEST_SENDER'");
    let receiver = env::var("TEST_RECEIVER").expect("Missing environment variable 'TEST_RECEIVER'");

    let temp_dir = tempdir().unwrap();
    let temp_path = temp_dir.path();
    assert!(temp_path.exists(), "Missing path: {}", temp_path.display());

    fs::copy("./test_data/message.yaml", temp_path.join("message.yaml")).unwrap();
    fs::copy("./test_data/test.pdf", temp_path.join("test.pdf")).unwrap();

    println!("Execute 'pigeon send --connection smtp'");
    let mut cmd = Command::cargo_bin("pigeon").unwrap();
    cmd.current_dir(temp_path);
    cmd.args([
        "send",
        &sender,
        &receiver,
        "--message-file",
        "./message.yaml",
        "--attachment",
        "./test.pdf",
        "--archive",
        "--archive-dir",
        "./my-sent-emails",
        "--display",
        "--assume-yes",
        "--connection",
        "smtp",
    ]);
    cmd.assert().success().stdout(
        str::contains("Reading message file './message.yaml' ...")
            .and(str::contains("Display message file:"))
            .and(str::contains("Display email:"))
            .and(str::contains("Sending email to 1 receiver ..."))
            .and(str::contains("Email sent"))
            .and(str::contains("Archiving"))
            .and(str::contains("Dry run:").not()),
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
    let sender = env::var("TEST_SENDER").expect("Missing environment variable 'TEST_SENDER'");
    let receiver = env::var("TEST_RECEIVER").expect("Missing environment variable 'TEST_RECEIVER'");

    let temp_dir = tempdir().unwrap();
    let temp_path = temp_dir.path();
    assert!(temp_path.exists(), "Missing path: {}", temp_path.display());

    fs::copy("./test_data/message.yaml", temp_path.join("message.yaml")).unwrap();
    fs::copy("./test_data/test.pdf", temp_path.join("test.pdf")).unwrap();

    println!("Execute 'pigeon send --connection aws ...'");
    let mut cmd = Command::cargo_bin("pigeon").unwrap();
    cmd.current_dir(temp_path);
    cmd.args([
        "send",
        &sender,
        &receiver,
        "--message-file",
        "./message.yaml",
        "--attachment",
        "./test.pdf",
        "--archive",
        "--archive-dir",
        "./my-sent-emails",
        "--display",
        "--assume-yes",
        "--connection",
        "aws",
    ]);
    cmd.assert().success().stdout(
        str::contains("Reading message file './message.yaml' ...")
            .and(str::contains("Display message file:"))
            .and(str::contains("Display email:"))
            .and(str::contains("Sending email to 1 receiver ..."))
            .and(str::contains("Email sent"))
            .and(str::contains("Archiving"))
            .and(str::contains("Dry run:").not()),
    );

    assert!(temp_path.join("my-sent-emails").exists());
}

#[test]
fn test_send_smtp_dry() {
    let temp_dir = tempdir().unwrap();
    let temp_path = temp_dir.path();
    assert!(temp_path.exists(), "Missing path: {}", temp_path.display());

    fs::copy("./test_data/message.yaml", temp_path.join("message.yaml")).unwrap();
    fs::copy("./test_data/test.pdf", temp_path.join("test.pdf")).unwrap();

    println!("Execute 'pigeon send --connection smtp'");
    let mut cmd = Command::cargo_bin("pigeon").unwrap();
    cmd.current_dir(temp_path);
    cmd.args([
        "send",
        "albert@einstein.com",
        "marie@curie.com",
        "--message-file",
        "./message.yaml",
        "--attachment",
        "./test.pdf",
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
        str::contains("Reading message file './message.yaml' ...")
            .and(str::contains("Display message file:"))
            .and(str::contains("Display email:"))
            .and(str::contains("Dry run: \u{1b}[32mactivated\u{1b}[0m"))
            .and(str::contains("Sending email to 1 receiver ..."))
            .and(str::contains(
                "marie@curie.com ... \u{1b}[32mdry run\u{1b}[0m",
            ))
            .and(str::contains("Archiving './my-sent-emails"))
            .and(str::contains("Email sent (dry run)")),
    );

    assert!(temp_path.join("my-sent-emails").exists());
}

#[test]
fn test_send_aws_dry() {
    let temp_dir = tempdir().unwrap();
    let temp_path = temp_dir.path();
    assert!(temp_path.exists(), "Missing path: {}", temp_path.display());

    fs::copy("./test_data/message.yaml", temp_path.join("message.yaml")).unwrap();
    fs::copy("./test_data/test.pdf", temp_path.join("test.pdf")).unwrap();

    println!("Execute 'pigeon send --connection aws ...'");
    let mut cmd = Command::cargo_bin("pigeon").unwrap();
    cmd.current_dir(temp_path);
    cmd.args([
        "send",
        "albert@einstein.com",
        "marie@curie.com",
        "--message-file",
        "./message.yaml",
        "--attachment",
        "./test.pdf",
        "--archive",
        "--archive-dir",
        "./my-sent-emails",
        "--display",
        "--assume-yes",
        "--connection",
        "aws",
        "--dry-run",
    ]);
    cmd.assert().success().stdout(
        str::contains("Reading message file './message.yaml' ...")
            .and(str::contains("Display message file:"))
            .and(str::contains("Display email:"))
            .and(str::contains("Dry run: \u{1b}[32mactivated\u{1b}[0m"))
            .and(str::contains("Sending email to 1 receiver ..."))
            .and(str::contains(
                "marie@curie.com ... \u{1b}[32mdry run\u{1b}[0m",
            ))
            .and(str::contains("Archiving './my-sent-emails"))
            .and(str::contains("Email sent (dry run)")),
    );

    assert!(temp_path.join("my-sent-emails").exists());
}
