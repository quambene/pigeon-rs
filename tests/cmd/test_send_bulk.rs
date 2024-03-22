use pigeon_rs::{app, cmd};

#[test]
#[ignore]
fn test_send_bulk_subject_content_dry() {
    let args = vec![
        cmd::BIN,
        cmd::SEND_BULK,
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
    ];

    let app = app();
    let matches = app.get_matches_from(args);
    let subcommand_matches = matches.subcommand_matches(cmd::SEND_BULK).unwrap();
    println!("subcommand matches: {:#?}", subcommand_matches);

    let res = cmd::send_bulk(subcommand_matches);
    println!("res: {:#?}", res);

    assert!(res.is_ok())
}

#[test]
#[ignore]
fn test_send_bulk_text_file_html_file_dry() {
    let args = vec![
        cmd::BIN,
        cmd::SEND_BULK,
        "albert@einstein.com",
        "--receiver-file",
        "./test_data/receiver.csv",
        "--subject",
        "Test Subject",
        "--text-file",
        "./test_data/message.txt",
        "--html-file",
        "./test_data/message.html",
        "--dry-run",
        "--display",
        "--assume-yes",
    ];

    let app = app();
    let matches = app.get_matches_from(args);
    let subcommand_matches = matches.subcommand_matches(cmd::SEND_BULK).unwrap();
    println!("subcommand matches: {:#?}", subcommand_matches);

    let res = cmd::send_bulk(subcommand_matches);
    println!("res: {:#?}", res);

    assert!(res.is_ok())
}

#[test]
#[ignore]
fn test_send_bulk_message_file_dry() {
    let args = vec![
        cmd::BIN,
        cmd::SEND_BULK,
        "albert@einstein.com",
        "--receiver-file",
        "./test_data/receiver.csv",
        "--message-file",
        "./test_data/message.yaml",
        "--dry-run",
        "--display",
        "--assume-yes",
    ];

    let app = app();
    let matches = app.get_matches_from(args);
    let subcommand_matches = matches.subcommand_matches(cmd::SEND_BULK).unwrap();
    println!("subcommand matches: {:#?}", subcommand_matches);

    let res = cmd::send_bulk(subcommand_matches);
    println!("res: {:#?}", res);

    assert!(res.is_ok())
}

#[test]
#[ignore]
fn test_send_bulk_receiver_column_dry() {
    let args = vec![
        cmd::BIN,
        cmd::SEND_BULK,
        "albert@einstein.com",
        "--receiver-file",
        "./test_data/contacts.csv",
        "--message-file",
        "./test_data/message.yaml",
        "--receiver-column",
        "contact",
        "--dry-run",
        "--display",
        "--assume-yes",
    ];

    let app = app();
    let matches = app.get_matches_from(args);
    let subcommand_matches = matches.subcommand_matches(cmd::SEND_BULK).unwrap();
    println!("subcommand matches: {:#?}", subcommand_matches);

    let res = cmd::send_bulk(subcommand_matches);
    println!("res: {:#?}", res);

    assert!(res.is_ok())
}

#[test]
#[ignore]
fn test_send_bulk_personalize_dry() {
    let args = vec![
        cmd::BIN,
        cmd::SEND_BULK,
        "albert@einstein.com",
        "--receiver-file",
        "./test_data/receiver.csv",
        "--message-file",
        "./test_data/message_personalized.yaml",
        "--personalize",
        "first_name",
        "last_name",
        "--dry-run",
        "--display",
        "--assume-yes",
    ];

    let app = app();
    let matches = app.get_matches_from(args);
    let subcommand_matches = matches.subcommand_matches(cmd::SEND_BULK).unwrap();
    println!("subcommand matches: {:#?}", subcommand_matches);

    let res = cmd::send_bulk(subcommand_matches);
    println!("res: {:#?}", res);

    assert!(res.is_ok())
}

#[test]
#[ignore]
fn test_archive_dry() {
    let args = vec![
        cmd::BIN,
        cmd::SEND_BULK,
        "albert@einstein.com",
        "--receiver-file",
        "./test_data/receiver.csv",
        "--message-file",
        "./test_data/message.yaml",
        "--dry-run",
        "--display",
        "--assume-yes",
        "--archive",
    ];

    let app = app();
    let matches = app.get_matches_from(args);
    let subcommand_matches = matches.subcommand_matches(cmd::SEND_BULK).unwrap();
    println!("subcommand matches: {:#?}", subcommand_matches);

    let res = cmd::send_bulk(subcommand_matches);
    println!("res: {:#?}", res);

    assert!(res.is_ok())
}

#[test]
#[ignore]
fn test_archive_dir_dry() {
    let args = vec![
        cmd::BIN,
        cmd::SEND_BULK,
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
    ];

    let app = app();
    let matches = app.get_matches_from(args);
    let subcommand_matches = matches.subcommand_matches(cmd::SEND_BULK).unwrap();
    println!("subcommand matches: {:#?}", subcommand_matches);

    let res = cmd::send_bulk(subcommand_matches);
    println!("res: {:#?}", res);

    assert!(res.is_ok())
}

#[test]
#[ignore]
fn test_attachment_pdf_dry() {
    let args = vec![
        cmd::BIN,
        cmd::SEND_BULK,
        "albert@einstein.com",
        "--receiver-file",
        "./test_data/receiver.csv",
        "--message-file",
        "./test_data/message.yaml",
        "--dry-run",
        "--display",
        "--assume-yes",
        "--archive",
        "--attachment",
        "./test_data/test.pdf",
    ];

    let app = app();
    let matches = app.get_matches_from(args);
    let subcommand_matches = matches.subcommand_matches(cmd::SEND_BULK).unwrap();
    println!("subcommand matches: {:#?}", subcommand_matches);

    let res = cmd::send_bulk(subcommand_matches);
    println!("res: {:#?}", res);

    assert!(res.is_ok())
}

#[test]
#[ignore]
fn test_attachment_png_dry() {
    let args = vec![
        cmd::BIN,
        cmd::SEND_BULK,
        "albert@einstein.com",
        "--receiver-file",
        "./test_data/receiver.csv",
        "--message-file",
        "./test_data/message.yaml",
        "--dry-run",
        "--display",
        "--assume-yes",
        "--archive",
        "--attachment",
        "./test_data/test.png",
    ];

    let app = app();
    let matches = app.get_matches_from(args);
    let subcommand_matches = matches.subcommand_matches(cmd::SEND_BULK).unwrap();
    println!("subcommand matches: {:#?}", subcommand_matches);

    let res = cmd::send_bulk(subcommand_matches);
    println!("res: {:#?}", res);

    assert!(res.is_ok())
}

#[test]
#[ignore]
fn test_attachment_odt_dry() {
    let args = vec![
        cmd::BIN,
        cmd::SEND_BULK,
        "albert@einstein.com",
        "--receiver-file",
        "./test_data/receiver.csv",
        "--message-file",
        "./test_data/message.yaml",
        "--dry-run",
        "--display",
        "--assume-yes",
        "--archive",
        "--attachment",
        "./test_data/test.odt",
    ];

    let app = app();
    let matches = app.get_matches_from(args);
    let subcommand_matches = matches.subcommand_matches(cmd::SEND_BULK).unwrap();
    println!("subcommand matches: {:#?}", subcommand_matches);

    let res = cmd::send_bulk(subcommand_matches);
    println!("res: {:#?}", res);

    assert!(res.is_ok())
}

#[test]
#[ignore]
fn test_send_bulk_aws_dry() {
    let args = vec![
        cmd::BIN,
        cmd::SEND_BULK,
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
    ];

    let app = app();
    let matches = app.get_matches_from(args);
    let subcommand_matches = matches.subcommand_matches(cmd::SEND_BULK).unwrap();
    println!("subcommand matches: {:#?}", subcommand_matches);

    let res = cmd::send_bulk(subcommand_matches);
    println!("res: {:#?}", res);

    assert!(res.is_ok())
}
