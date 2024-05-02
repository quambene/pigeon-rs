use crate::{arg, cmd, sources::ConnVars};
use anyhow::{anyhow, Result};
use clap::ArgMatches;
use postgres::{Client, NoTls, SimpleQueryMessage};

pub fn simple_query(matches: &ArgMatches) -> Result<(), anyhow::Error> {
    if matches.get_flag(arg::VERBOSE) {
        println!("matches: {:#?}", matches);
    }

    if matches.contains_id(cmd::SIMPLE_QUERY) {
        let conn_vars = ConnVars::from_env()?;
        let simple_query = match matches.get_one::<String>(cmd::SIMPLE_QUERY) {
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
