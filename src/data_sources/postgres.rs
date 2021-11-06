use anyhow::{Context, Result};
use clap::ArgMatches;
use connectorx::{
    destinations::arrow::ArrowDestination,
    prelude::{Dispatcher, PostgresArrowTransport},
    sources::postgres::{rewrite_tls_args, BinaryProtocol, PostgresSource},
    sql::CXQuery,
};
use polars_core::frame::DataFrame;
use postgres::NoTls;
use std::{env, fmt};

use crate::{arg, data_sources::SshTunnel};

pub struct Password(pub String);

impl fmt::Debug for Password {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let len = self.0.len();
        write!(f, "{}", &("*".repeat(len - 3) + &self.0[(len - 3)..]))
    }
}

impl fmt::Display for Password {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

#[derive(Debug)]
pub struct ConnVars {
    pub db_host: String,
    pub db_port: String,
    pub db_name: String,
    pub db_user: String,
    pub db_password: Password,
}

impl ConnVars {
    pub fn from_env() -> Result<Self, anyhow::Error> {
        println!("Reading env vars ...");

        let db_host = env::var("DB_HOST").context("Missing environment variable 'DB_URL'")?;
        let db_port = env::var("DB_PORT").context("Missing environment variable 'DB_PORT'")?;
        let db_user = env::var("DB_USER").context("Missing environment variable 'DB_USER'")?;
        let db_password = Password(
            env::var("DB_PASSWORD").context("Missing environment variable 'DB_PASSWORD'")?,
        );
        let db_name = env::var("DB_NAME").context("Missing environment variable 'DB_NAME'")?;

        let conn_vars = ConnVars {
            db_host,
            db_port,
            db_name,
            db_user,
            db_password,
        };

        println!("Using these environment variables: {:#?}", &conn_vars);

        Ok(conn_vars)
    }

    pub fn connection_url(&self) -> String {
        String::from("postgresql://")
            + &self.db_user
            + ":"
            + &self.db_password.0
            + "@"
            + &self.db_host
            + ":"
            + &self.db_port
            + "/"
            + &self.db_name
    }
}

pub fn query_postgres(matches: &ArgMatches<'_>, query: String) -> Result<DataFrame, anyhow::Error> {
    let conn_vars = ConnVars::from_env()?;
    let connection_url: String;
    let ssh_tunnel: Option<SshTunnel>;

    if matches.is_present(arg::SSH_TUNNEL) {
        ssh_tunnel = Some(SshTunnel::new(matches, &conn_vars)?);
    } else {
        ssh_tunnel = None;
    }

    match &ssh_tunnel {
        Some(tunnel) => connection_url = tunnel.url.to_string(),
        None => connection_url = conn_vars.connection_url(),
    }

    let (config, _tls) = rewrite_tls_args(&connection_url)?;
    let source = PostgresSource::<BinaryProtocol, NoTls>::new(config, NoTls, 10)?;
    let mut destination = ArrowDestination::new();
    let queries = &[CXQuery::naked(query)];
    let dispatcher = Dispatcher::<_, _, PostgresArrowTransport<BinaryProtocol, NoTls>>::new(
        source,
        &mut destination,
        queries,
    );
    dispatcher.run()?;
    let df = destination.polars();

    match &ssh_tunnel {
        Some(tunnel) => tunnel.kill()?,
        None => (),
    }

    Ok(df?)
}
