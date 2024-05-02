use crate::{
    arg::{self, val},
    cmd,
    email_provider::AwsSesClient,
    email_transmission::SmtpClient,
};
use anyhow::{anyhow, Result};
use clap::ArgMatches;

pub fn connect(matches: &ArgMatches) -> Result<(), anyhow::Error> {
    if matches.get_flag(arg::VERBOSE) {
        println!("matches: {:#?}", matches);
    }

    if matches.contains_id(cmd::CONNECT) {
        match matches.get_one::<String>(cmd::CONNECT) {
            Some(connection) => match connection.to_lowercase().as_str() {
                val::SMTP => {
                    let _client = SmtpClient::new()?;
                    Ok(())
                }
                val::AWS => {
                    let _client = AwsSesClient::new(matches)?;
                    Ok(())
                }
                other => Err(anyhow!(
                    "Value '{}' for argument '{}' not supported",
                    other,
                    cmd::CONNECT
                )),
            },
            None => Err(anyhow!("Missing value for argument '{}'", cmd::CONNECT)),
        }
    } else {
        Err(anyhow!("Missing argument '{}'", cmd::CONNECT))
    }
}
