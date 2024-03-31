use assert_cmd::Command;
use predicates::{boolean::PredicateBooleanExt, str};

/// This test requires environment variables `SMTP_SERVER`, `SMTP_USERNAME`, and
/// `SMTP_PASSWORD`.
#[test]
#[ignore]
fn test_connect_smtp() {
    println!("Execute 'pigeon connect smtp'");
    let mut cmd = Command::cargo_bin("pigeon").unwrap();
    cmd.args(["connect", "smtp"]);
    cmd.assert()
        .success()
        .stdout(str::contains("Connecting to SMTP server").and(str::contains("ok")));
}

/// This test requires environment variables `AWS_ACCESS_KEY_ID`,
/// `AWS_SECRET_ACCESS_KEY`, and `AWS_REGION`.
#[test]
#[ignore]
fn test_connect_aws() {
    println!("Execute 'pigeon connect aws'");
    let mut cmd = Command::cargo_bin("pigeon").unwrap();
    cmd.args(["connect", "aws"]);
    cmd.assert()
        .success()
        .stdout(str::contains("Connecting to aws server in region").and(str::contains("ok")));
}
