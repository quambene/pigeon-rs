use pigeon_rs::{app, cmd};
use std::env;

#[test]
#[ignore]
fn test_display_query() {
    let test_query = env::var("TEST_QUERY").expect("Missing environment variable 'TEST_QUERY'");
    let args = vec![cmd::BIN, cmd::QUERY, test_query.as_str(), "--display"];

    let app = app();
    let matches = app.get_matches_from(args);
    let subcommand_matches = matches.subcommand_matches(cmd::QUERY).unwrap();
    println!("subcommand matches: {:#?}", subcommand_matches);

    let res = cmd::query(subcommand_matches);
    println!("res: {:#?}", res);

    assert!(res.is_ok())
}

#[test]
#[ignore]
fn test_save_query() {
    let test_query = env::var("TEST_QUERY").expect("Missing environment variable 'TEST_QUERY'");
    let args = vec![
        cmd::BIN,
        cmd::QUERY,
        test_query.as_str(),
        "--display",
        "--save",
    ];

    let app = app();
    let matches = app.get_matches_from(args);
    let subcommand_matches = matches.subcommand_matches(cmd::QUERY).unwrap();
    println!("subcommand matches: {:#?}", subcommand_matches);

    let res = cmd::query(subcommand_matches);
    println!("res: {:#?}", res);

    assert!(res.is_ok())
}

#[test]
#[ignore]
fn test_save_dir() {
    let test_query = env::var("TEST_QUERY").expect("Missing environment variable 'TEST_QUERY'");
    let args = vec![
        cmd::BIN,
        cmd::QUERY,
        test_query.as_str(),
        "--display",
        "--save",
        "--save-dir",
        "./my-saved-queries",
    ];

    let app = app();
    let matches = app.get_matches_from(args);
    let subcommand_matches = matches.subcommand_matches(cmd::QUERY).unwrap();
    println!("subcommand matches: {:#?}", subcommand_matches);

    let res = cmd::query(subcommand_matches);
    println!("res: {:#?}", res);

    assert!(res.is_ok())
}
