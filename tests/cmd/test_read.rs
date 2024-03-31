use assert_cmd::Command;
use predicates::{boolean::PredicateBooleanExt, str};

#[test]
fn test_read() {
    let test_data = "./test_data/receiver.csv";
    println!("Execute 'pigeon read {test_data}'");
    let mut cmd = Command::cargo_bin("pigeon").unwrap();
    cmd.args(["read", test_data]);
    cmd.assert().success().stdout(str::contains(
        "Reading csv file './test_data/receiver.csv' ...",
    ));
}

#[test]
fn test_read_display() {
    let test_data = "./test_data/receiver.csv";
    println!("Execute 'pigeon read {test_data} --display'");
    let mut cmd = Command::cargo_bin("pigeon").unwrap();
    cmd.args(["read", test_data, "--display"]);
    cmd.assert().success().stdout(
        str::contains("Reading csv file './test_data/receiver.csv' ...").and(str::contains(
            "Display csv file: shape: (2, 3)
┌────────────┬──────────────┬────────────────────────────┐
│ first_name ┆ last_name    ┆ email                      │
│ ---        ┆ ---          ┆ ---                        │
│ str        ┆ str          ┆ str                        │
╞════════════╪══════════════╪════════════════════════════╡
│ Marie      ┆ Curie        ┆ marie@curie.com            │
│ Alexandre  ┆ Grothendieck ┆ alexandre@grothendieck.com │
└────────────┴──────────────┴────────────────────────────┘",
        )),
    );
}
