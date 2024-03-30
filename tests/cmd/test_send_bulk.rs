use assert_cmd::Command;
use predicates::{boolean::PredicateBooleanExt, str};
use tempfile::tempdir;

#[test]
fn test_send_bulk_smtp_dry() {
    let temp_dir = tempdir().unwrap();
    let temp_path = temp_dir.path();
    assert!(temp_path.exists(), "Missing path: {}", temp_path.display());

    println!("Execute 'pigeon send-bulk'");
    let mut cmd = Command::cargo_bin("pigeon").unwrap();
    cmd.current_dir(temp_path);
    cmd.args([
        "send-bulk",
        "albert@einstein.com",
        "--receiver-file",
        "./test_data/receiver.csv",
        "--subject",
        "Test Subject",
        "--content",
        "This is a test message (plaintext).",
        "--archive",
        "--archive-dir",
        "./my-sent-emails",
        "--display",
        "--assume-yes",
        "--dry-run",
    ]);
    cmd.assert().success().stdout(
        str::contains("Reading csv file './test_data/receiver.csv' ...")
            .and(str::contains("Display csv file:").and(str::contains("Display emails:"))),
    );

    assert!(temp_path.join("my-sent-emails").exists());
}

#[test]
fn test_send_bulk_aws_dry() {
    let temp_dir = tempdir().unwrap();
    let temp_path = temp_dir.path();
    assert!(temp_path.exists(), "Missing path: {}", temp_path.display());

    println!("Execute 'pigeon send-bulk'");
    let mut cmd = Command::cargo_bin("pigeon").unwrap();
    cmd.current_dir(temp_path);
    cmd.args([
        "send-bulk",
        "albert@einstein.com",
        "--receiver-file",
        "./test_data/receiver.csv",
        "--message-file",
        "./test_data/message.yaml",
        "--archive",
        "--archive-dir",
        "./my-sent-emails",
        "--display",
        "--assume-yes",
        "--connection",
        "aws",
        "--dry-run",
    ]);
    cmd.assert().success();

    assert!(temp_path.join("my-sent-emails").exists());
}
