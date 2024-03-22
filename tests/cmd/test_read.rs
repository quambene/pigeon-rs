use pigeon_rs::{app, cmd};

#[test]
fn test_read() {
    let args = vec![cmd::BIN, cmd::READ, "./test_data/receiver.csv"];

    let app = app();
    let matches = app.get_matches_from(args);
    let subcommand_matches = matches.subcommand_matches(cmd::READ).unwrap();
    println!("subcommand matches: {:#?}", subcommand_matches);

    let res = cmd::read(subcommand_matches);
    println!("res: {:#?}", res);

    assert!(res.is_ok())
}
