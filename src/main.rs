use anyhow::anyhow;
use pigeon_rs::{app, arg, cmd};

fn main() -> Result<(), anyhow::Error> {
    let app = app();
    let matches = app.get_matches();

    if matches.is_present(arg::VERBOSE) {
        println!("matches: {:#?}", matches);
    }

    match matches.subcommand() {
        (cmd::INIT, Some(matches)) => cmd::init(matches),
        (cmd::CONNECT, Some(matches)) => cmd::connect(matches),
        (cmd::QUERY, Some(matches)) => cmd::query(matches),
        (cmd::SIMPLE_QUERY, Some(matches)) => cmd::simple_query(matches),
        (cmd::READ, Some(matches)) => cmd::read(matches),
        (cmd::SEND, Some(matches)) => cmd::send(matches),
        (cmd::SEND_BULK, Some(matches)) => cmd::send_bulk(matches),
        (_, _) => Err(anyhow!("Subcommand not found")),
    }
}
