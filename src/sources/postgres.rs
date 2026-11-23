use super::SshTunnel;
use anyhow::{anyhow, Context, Result};
use chrono::{DateTime, NaiveDate, NaiveDateTime, Utc};
use polars::{
    frame::DataFrame,
    prelude::{Column, DataType, TimeUnit},
};
use postgres::{
    types::{FromSql, Type},
    Client, NoTls, Row,
};
use std::{env, fmt};
use url::Url;

pub struct Password(pub String);

impl fmt::Debug for Password {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let len = self.0.len();
        write!(f, "{}", ("*".repeat(len - 3) + &self.0[(len - 3)..]))
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

        println!("Using these environment variables: {:#?}", conn_vars);

        Ok(conn_vars)
    }

    pub fn connection_url(&self) -> Result<Url, anyhow::Error> {
        let connection_url = format!(
            "postgresql://{}:{}@{}:{}/{}",
            self.db_user, self.db_password.0, self.db_host, self.db_port, self.db_name
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
    let df = run_query(&connection.url, query);

    if let Some(tunnel) = &connection.ssh_tunnel {
        tunnel.kill()?;
    }

    df
}

fn run_query(url: &Url, query: &str) -> Result<DataFrame, anyhow::Error> {
    let mut client = Client::connect(url.as_str(), NoTls).context("Can't connect to database")?;
    let statement = client.prepare(query).context("Can't prepare query")?;
    let rows = client
        .query(&statement, &[])
        .context("Can't execute query")?;

    let columns = statement
        .columns()
        .iter()
        .enumerate()
        .map(|(index, column)| to_column(column.name(), column.type_(), &rows, index))
        .collect::<Result<Vec<_>, _>>()?;

    let df = DataFrame::new(columns)?;
    Ok(df)
}

/// Convert the values of the column at `index` into a polars column.
fn to_column(name: &str, ty: &Type, rows: &[Row], index: usize) -> Result<Column, anyhow::Error> {
    let column = match *ty {
        Type::BOOL => Column::new(name.into(), values::<bool>(rows, index)?),
        Type::INT2 => Column::new(
            name.into(),
            map_values(values::<i16>(rows, index)?, i32::from),
        ),
        Type::INT4 => Column::new(name.into(), values::<i32>(rows, index)?),
        Type::INT8 => Column::new(name.into(), values::<i64>(rows, index)?),
        Type::OID => Column::new(name.into(), values::<u32>(rows, index)?),
        Type::FLOAT4 => Column::new(name.into(), values::<f32>(rows, index)?),
        Type::FLOAT8 => Column::new(name.into(), values::<f64>(rows, index)?),
        Type::BYTEA => Column::new(name.into(), values::<&[u8]>(rows, index)?),
        Type::DATE => Column::new(name.into(), values::<NaiveDate>(rows, index)?),
        Type::TIMESTAMP => datetime_column(
            name,
            map_values(values::<NaiveDateTime>(rows, index)?, |value| value.and_utc()),
        )?,
        Type::TIMESTAMPTZ => datetime_column(name, values::<DateTime<Utc>>(rows, index)?)?,
        Type::UUID => Column::new(
            name.into(),
            map_values(values::<uuid::Uuid>(rows, index)?, |value| {
                value.to_string()
            }),
        ),
        ref ty if <&str as FromSql>::accepts(ty) => {
            Column::new(name.into(), values::<&str>(rows, index)?)
        }
        ref ty => {
            return Err(anyhow!(
                "Unsupported type '{ty}' for column '{name}'. Cast the column in your query, e.g. '{name}::text'"
            ))
        }
    };

    Ok(column)
}

fn values<'a, T>(rows: &'a [Row], index: usize) -> Result<Vec<Option<T>>, anyhow::Error>
where
    T: FromSql<'a>,
{
    rows.iter()
        .map(|row| row.try_get::<_, Option<T>>(index))
        .collect::<Result<Vec<_>, _>>()
        .context("Can't read column value")
}

fn map_values<T, U>(values: Vec<Option<T>>, f: impl Fn(T) -> U) -> Vec<Option<U>> {
    values.into_iter().map(|value| value.map(&f)).collect()
}

/// Create a datetime column with microsecond precision (as stored by Postgres).
fn datetime_column(
    name: &str,
    values: Vec<Option<DateTime<Utc>>>,
) -> Result<Column, anyhow::Error> {
    let micros = map_values(values, |value| value.timestamp_micros());
    let column =
        Column::new(name.into(), micros).cast(&DataType::Datetime(TimeUnit::Microseconds, None))?;
    Ok(column)
}
