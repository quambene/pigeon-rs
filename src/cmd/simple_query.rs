use crate::{arg, cmd, data_sources::ConnVars};
use anyhow::{anyhow, Result};
use clap::{Arg, ArgMatches};
use postgres::{Client, NoTls, SimpleQueryMessage};

pub fn simple_query_args() -> [Arg<'static, 'static>; 2] {
    [
        Arg::with_name(cmd::SIMPLE_QUERY)
            .index(1)
            .required(true)
            .takes_value(true)
            .help("Takes a sql query"),
        Arg::with_name(arg::VERBOSE)
            .long(arg::VERBOSE)
            .takes_value(false)
            .help("Shows what is going on for subcommand"),
    ]
}

pub fn simple_query(matches: &ArgMatches) -> Result<(), anyhow::Error> {
    if matches.is_present(arg::VERBOSE) {
        println!("matches: {:#?}", matches);
    }

    if matches.is_present(cmd::SIMPLE_QUERY) {
        let conn_vars = ConnVars::from_env()?;
        let simple_query = match matches.value_of(cmd::SIMPLE_QUERY) {
            Some(query) => query,
            None => {
                return Err(anyhow!(
                    "Missing value for argument '{}'",
                    cmd::SIMPLE_QUERY
                ))
            }
        };
        let mut client = Client::connect(
            &format!(
                "host={} port={} user={} password={}",
                &conn_vars.db_host, &conn_vars.db_port, &conn_vars.db_user, &conn_vars.db_password
            ),
            NoTls,
        )?;
        let rows = client.simple_query(simple_query)?;

        for row in rows {
            println!("row: ");
            match &row {
                SimpleQueryMessage::Row(simple_query_row) => {
                    for i in 0..simple_query_row.len() {
                        print!("{:#?} ", simple_query_row.get(i))
                    }
                }
                SimpleQueryMessage::CommandComplete(row_count) => {
                    println!("row count: {}", row_count)
                }
                _ => unreachable!(),
            }
        }

        Ok(())
    } else {
        Err(anyhow!("Missing argument '{}'", cmd::SIMPLE_QUERY))
    }
}
