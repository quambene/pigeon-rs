use assert_cmd::Command;
use predicates::{boolean::PredicateBooleanExt, str};
use std::fs;
use tempfile::tempdir;

#[test]
fn test_send_bulk_smtp_dry() {
    let temp_dir = tempdir().unwrap();
    let temp_path = temp_dir.path();
    assert!(temp_path.exists(), "Missing path: {}", temp_path.display());

    fs::copy("./test_data/receiver.csv", temp_path.join("receiver.csv")).unwrap();
    fs::copy("./test_data/message.yaml", temp_path.join("message.yaml")).unwrap();
    fs::copy("./test_data/test.pdf", temp_path.join("test.pdf")).unwrap();

    println!("Execute 'pigeon send-bulk'");
    let mut cmd = Command::cargo_bin("pigeon").unwrap();
    cmd.current_dir(temp_path);
    cmd.args([
        "send-bulk",
        "albert@einstein.com",
        "--receiver-file",
        "./receiver.csv",
        "--message-file",
        "./message.yaml",
        "--attachment",
        "./test.pdf",
        "--archive",
        "--archive-dir",
        "./my-sent-emails",
        "--display",
        "--assume-yes",
        "--dry-run",
    ]);
    cmd.assert().success().stdout(
        str::contains("Reading csv file './receiver.csv' ...")
            .and(str::contains("Display csv file:").and(str::contains("Display emails:"))),
    );

    assert!(temp_path.join("my-sent-emails").exists());
}

#[test]
fn test_send_bulk_aws_dry() {
    let temp_dir = tempdir().unwrap();
    let temp_path = temp_dir.path();
    assert!(temp_path.exists(), "Missing path: {}", temp_path.display());

    fs::copy("./test_data/receiver.csv", temp_path.join("receiver.csv")).unwrap();
    fs::copy("./test_data/message.yaml", temp_path.join("message.yaml")).unwrap();
    fs::copy("./test_data/test.pdf", temp_path.join("test.pdf")).unwrap();

    println!("Execute 'pigeon send-bulk'");
    let mut cmd = Command::cargo_bin("pigeon").unwrap();
    cmd.current_dir(temp_path);
    cmd.args([
        "send-bulk",
        "albert@einstein.com",
        "--receiver-file",
        "./receiver.csv",
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
        str::contains("Reading csv file './receiver.csv' ...")
            .and(str::contains("Display csv file:").and(str::contains("Display emails:"))),
    );

    assert!(temp_path.join("my-sent-emails").exists());
}
