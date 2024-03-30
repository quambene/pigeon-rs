use assert_cmd::Command;
use predicates::{boolean::PredicateBooleanExt, str};

#[test]
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
fn test_send_bulk_subject_content() {
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
        "--display",
        "--assume-yes",
    ]);
    cmd.assert().success().stdout(
        str::contains("Reading csv file './test_data/receiver.csv' ...")
            .and(str::contains("Display csv file:").and(str::contains("Display emails:"))),
    );
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
        "aws",
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
