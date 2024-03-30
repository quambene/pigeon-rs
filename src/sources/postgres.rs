use super::SshTunnel;
use anyhow::{Context, Result};
use connectorx::{
    destinations::arrow2::Arrow2Destination,
    prelude::{Dispatcher, PostgresArrow2Transport},
    sources::postgres::{rewrite_tls_args, BinaryProtocol, PostgresSource},
    sql::CXQuery,
};
use polars::frame::DataFrame;
use postgres::NoTls;
use std::{env, fmt};
use url::Url;

pub struct Password(pub String);

impl fmt::Debug for Password {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let len = self.0.len();
        write!(f, "{}", &("*".repeat(len - 3) + &self.0[(len - 3)..]))
    }
}

impl fmt::Display for Password {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
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

    pub fn connection_url(&self) -> Result<Url, anyhow::Error> {
        let connection_url = format!(
            "postgresql://{}:{}@{}:{}/{}",
            &self.db_user, &self.db_password.0, &self.db_host, &self.db_port, &self.db_name
        );
        let url = Url::parse(&connection_url)?;
        Ok(url)
    }
}

pub struct DbConnection {
    /// The connection url of the DB.
    url: Url,
    /// Connection via ssh tunnel.
    ssh_tunnel: Option<SshTunnel>,
}

impl DbConnection {
    pub fn new(conn_vars: &ConnVars, ssh_tunnel: Option<&str>) -> Result<Self, anyhow::Error> {
        let connection = if let Some(tunnel) = ssh_tunnel {
            let ssh_tunnel = SshTunnel::new(tunnel, conn_vars)?;
            Self {
                url: ssh_tunnel.connection_url().to_owned(),
                ssh_tunnel: Some(ssh_tunnel),
            }
        } else {
            let connection_url = conn_vars.connection_url()?;
            Self {
                url: connection_url,
                ssh_tunnel: None,
            }
        };
        Ok(connection)
    }
}

pub fn query_postgres(connection: &DbConnection, query: &str) -> Result<DataFrame, anyhow::Error> {
    let (config, _tls) = rewrite_tls_args(&connection.url)?;
    let source = PostgresSource::<BinaryProtocol, NoTls>::new(config, NoTls, 3)?;
    let mut destination = Arrow2Destination::new();
    let queries = &[CXQuery::naked(query)];
    let dispatcher = Dispatcher::<_, _, PostgresArrow2Transport<BinaryProtocol, NoTls>>::new(
        source,
        &mut destination,
        queries,
        None,
    );
    dispatcher.run()?;
    let df = destination.polars()?;

    if let Some(tunnel) = &connection.ssh_tunnel {
        tunnel.kill()?;
    }

    Ok(df)
}
