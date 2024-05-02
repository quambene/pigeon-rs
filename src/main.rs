use anyhow::anyhow;
use pigeon_rs::{app, arg, cmd};

fn main() -> Result<(), anyhow::Error> {
    let app = app();
    let matches = app.get_matches();

    if matches.get_flag(arg::VERBOSE) {
        println!("matches: {:#?}", matches);
    }

    match matches.subcommand() {
        Some((cmd::INIT, matches)) => cmd::init(matches),
        Some((cmd::CONNECT, matches)) => cmd::connect(matches),
        Some((cmd::QUERY, matches)) => cmd::query(matches),
        Some((cmd::SIMPLE_QUERY, matches)) => cmd::simple_query(matches),
        Some((cmd::READ, matches)) => cmd::read(matches),
        Some((cmd::SEND, matches)) => cmd::send(matches),
        Some((cmd::SEND_BULK, matches)) => cmd::send_bulk(matches),
        _ => Err(anyhow!("Subcommand not found")),
    }
}
