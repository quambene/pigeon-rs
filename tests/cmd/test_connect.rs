use assert_cmd::Command;
use predicates::{boolean::PredicateBooleanExt, str};

#[test]
fn test_connect_smtp() {
    println!("Execute 'pigeon connect smtp'");
    let mut cmd = Command::cargo_bin("pigeon").unwrap();
    cmd.args(["connect", "smtp"]);
    cmd.assert()
        .success()
        .stdout(str::contains("Connecting to SMTP server").and(str::contains("ok")));
}

#[test]
fn test_connect_aws() {
    println!("Execute 'pigeon connect aws'");
    let mut cmd = Command::cargo_bin("pigeon").unwrap();
    cmd.args(["connect", "aws"]);
    cmd.assert()
        .success()
        .stdout(str::contains("Connecting to aws server in region").and(str::contains("ok")));
}
