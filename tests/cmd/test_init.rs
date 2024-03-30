use assert_cmd::Command;
use predicates::{boolean::PredicateBooleanExt, str};
use tempfile::tempdir;

#[test]
fn test_init() {
    let temp_dir = tempdir().unwrap();
    let temp_path = temp_dir.path();
    assert!(temp_path.exists(), "Missing path: {}", temp_path.display());

    println!("Execute 'pigeon init'");
    let mut cmd = Command::cargo_bin("pigeon").unwrap();
    cmd.current_dir(temp_path);
    cmd.args(["init"]);
    cmd.assert().success().stdout(
        str::contains("Creating message template in current directory:").and(str::contains(
            format!("{}/message.yaml", temp_path.display()),
        )),
    );
}
