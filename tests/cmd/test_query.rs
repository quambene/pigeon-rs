use assert_cmd::Command;
use predicates::str;
use std::{env, fs};
use tempfile::tempdir;

#[test]
#[ignore]
fn test_query_display() {
    let test_query = env::var("TEST_QUERY").expect("Missing environment variable 'TEST_QUERY'");
    println!("Execute 'pigeon query {test_query} --display'");
    let mut cmd = Command::cargo_bin("pigeon").unwrap();
    cmd.args(["query", test_query.as_str(), "--display"]);
    cmd.assert()
        .success()
        .stdout(str::contains("Display query result"));
}

#[test]
#[ignore]
fn test_query_save() {
    let test_query = env::var("TEST_QUERY").expect("Missing environment variable 'TEST_QUERY'");
    let temp_dir = tempdir().unwrap();
    let temp_path = temp_dir.path();
    assert!(temp_path.exists(), "Missing path: {}", temp_path.display());
    let save_dir = temp_path.to_str().unwrap();

    println!("Execute 'pigeon query {test_query} --save'");
    let mut cmd = Command::cargo_bin("pigeon").unwrap();
    cmd.current_dir(save_dir);
    cmd.args(["query", test_query.as_str(), "--save"]);
    cmd.assert()
        .success()
        .stdout(str::contains("Save query result to file"));

    if let Ok(mut entries) = fs::read_dir(temp_path.join("saved_queries")) {
        let dir_entry = entries.find_map(|entry| {
            if let Ok(entry) = entry {
                if entry.file_name().to_str().is_some_and(|file_name| {
                    file_name.contains("query") && file_name.ends_with(".csv")
                }) {
                    return Some(entry);
                }
            }

            None
        });
        assert!(dir_entry.is_some());
    }
}

#[test]
#[ignore]
fn test_query_save_dir() {
    let test_query = env::var("TEST_QUERY").expect("Missing environment variable 'TEST_QUERY'");
    let temp_dir = tempdir().unwrap();
    let temp_path = temp_dir.path();
    assert!(temp_path.exists(), "Missing path: {}", temp_path.display());
    let save_dir = temp_path.to_str().unwrap();

    println!("Execute 'pigeon query {test_query} --save --save-dir {save_dir}'");
    let mut cmd = Command::cargo_bin("pigeon").unwrap();
    cmd.args([
        "query",
        test_query.as_str(),
        "--save",
        "--save-dir",
        save_dir,
    ]);
    cmd.assert()
        .success()
        .stdout(str::contains("Save query result to file"));

    if let Ok(mut entries) = fs::read_dir(temp_path) {
        let dir_entry = entries.find_map(|entry| {
            if let Ok(entry) = entry {
                if entry.file_name().to_str().is_some_and(|file_name| {
                    file_name.contains("query") && file_name.ends_with(".csv")
                }) {
                    return Some(entry);
                }
            }

            None
        });
        assert!(dir_entry.is_some());
    }
}
