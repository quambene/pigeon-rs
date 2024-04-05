/* These tests requires the following environment variables:
     - DB_HOST
     - DB_PORT
     - DB_USER
     - DB_PASSWORD
     - DB_NAME
*/

use assert_cmd::Command;
use predicates::str;
use std::fs;
use tempfile::tempdir;

#[test]
fn test_query_display() {
    let test_query = "select email, first_name, last_name from account";

    println!("Execute 'pigeon query {test_query} --display'");
    let mut cmd = Command::cargo_bin("pigeon").unwrap();
    cmd.args(["query", test_query, "--display"]);
    cmd.assert().success().stdout(str::contains(
        "Display query result: shape: (2, 3)
┌────────────────────────────┬────────────┬──────────────┐
│ email                      ┆ first_name ┆ last_name    │
│ ---                        ┆ ---        ┆ ---          │
│ str                        ┆ str        ┆ str          │
╞════════════════════════════╪════════════╪══════════════╡
│ marie@curie.com            ┆ Marie      ┆ Curie        │
│ alexandre@grothendieck.com ┆ Alexandre  ┆ Grothendieck │
└────────────────────────────┴────────────┴──────────────┘",
    ));
}

#[test]
fn test_query_save_csv() {
    let test_query = "select email, first_name, last_name from account";
    let temp_dir = tempdir().unwrap();
    let temp_path = temp_dir.path();
    assert!(temp_path.exists(), "Missing path: {}", temp_path.display());
    let save_dir = temp_path.to_str().unwrap();

    println!("Execute 'pigeon query {test_query} --save --save-dir {save_dir}'");
    let mut cmd = Command::cargo_bin("pigeon").unwrap();
    cmd.args(["query", test_query, "--save", "--save-dir", save_dir]);
    cmd.assert()
        .success()
        .stdout(str::contains("Save query result to file:"));

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

#[test]
fn test_query_save_png() {
    let test_query = "select id, image from images";
    let temp_dir = tempdir().unwrap();
    let temp_path = temp_dir.path();
    assert!(temp_path.exists(), "Missing path: {}", temp_path.display());
    let save_dir = temp_path.to_str().unwrap();
    let file_type = "png";
    let image_column = "image";
    let image_name = "id";

    println!(
        "Execute 'pigeon query {test_query} --save --save-dir {save_dir} --file-type {file_type} --image-column {image_column} --image-name {image_name}'"
    );
    let mut cmd = Command::cargo_bin("pigeon").unwrap();
    cmd.args([
        "query",
        test_query,
        "--save",
        "--save-dir",
        save_dir,
        "--file-type",
        file_type,
        "--image-column",
        image_column,
        "--image-name",
        image_name,
    ]);
    cmd.assert().success().stdout(str::contains(format!(
        "Save query result to file: {}/1.png",
        temp_path.display()
    )));
}
