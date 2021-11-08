use crate::{arg, data_sources::postgres::ConnVars};
use anyhow::{anyhow, Context, Result};
use clap::ArgMatches;
use std::{
    env,
    process::{Child, Command},
};

pub struct SshTunnel {
    process: Child,
    pub connection_url: String,
}

const LOCALHOST: &str = "127.0.0.1";

impl SshTunnel {
    pub fn new(matches: &ArgMatches<'_>, conn_vars: &ConnVars) -> Result<Self, anyhow::Error> {
        println!("Opening ssh tunnel ...");

        let server_host = env::var("SERVER_HOST")
            .context("Missing environment variable 'SERVER_HOST'. Needed for ssh tunnel.")?;
        let server_user = env::var("SERVER_USER")
            .context("Missing environment variable 'SERVER_USER'. Needed for ssh tunnel.")?;

        let local_port = match matches.value_of(arg::SSH_TUNNEL) {
            Some(port) => port,
            None => return Err(anyhow!("Missing value for argument '{}'", arg::SSH_TUNNEL)),
        };
        let local_url = &(LOCALHOST.to_string() + ":" + local_port) as &str;
        let db_url = conn_vars.db_host.to_string() + ":" + &conn_vars.db_port;

        let port_fwd = local_url.to_string() + ":" + &db_url;
        let ssh_connection = server_user.to_string() + "@" + &server_host;

        let process = Command::new("ssh")
            .args(&["-N", "-T", "-L", &port_fwd, &ssh_connection])
            .spawn()?;

        let connection_url = SshTunnel::connection_url(conn_vars, local_url);
        let ssh_tunnel = SshTunnel {
            process,
            connection_url,
        };

        println!(
            "Ssh tunnel openend: forwarding port from {} to {} for user {} on server {}",
            local_url, db_url, server_user, server_host
        );
        Ok(ssh_tunnel)
    }

    pub fn kill(&self) -> Result<(), anyhow::Error> {
        let pid = self.process.id();
        let mut cmd = Command::new("kill");

        match cmd.arg(pid.to_string()).spawn() {
            Ok(mut child) => {
                let exit_status = child.wait().context(format!(
                    "Unable to kill ssh tunnel. Please kill PID {} manually.",
                    pid
                ))?;

                if exit_status.success() {
                    println!("Closing ssh tunnel ...");
                    Ok(())
                } else {
                    Err(anyhow!(
                        "Unable to kill ssh tunnel. Please kill PID {} manually.",
                        pid
                    ))
                }
            }
            Err(err) => Err(anyhow!(err).context(format!(
                "Can't kill ssh tunnel. Please kill PID {} manually.",
                pid
            ))),
        }
    }

    fn connection_url(conn_vars: &ConnVars, tunnel_url: &str) -> String {
        String::from("postgresql://")
            + &conn_vars.db_user
            + ":"
            + &conn_vars.db_password.0
            + "@"
            + tunnel_url
            + "/"
            + &conn_vars.db_name
    }
}
