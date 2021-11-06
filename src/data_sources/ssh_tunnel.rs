use crate::{arg, data_sources::postgres::ConnVars};
use anyhow::{anyhow, Context, Result};
use clap::ArgMatches;
use std::process::{Child, Command};

pub struct SshTunnel {
    process: Child,
    pub url: String,
}

const LOCALHOST: &str = "127.0.0.1";

impl SshTunnel {
    pub fn new(matches: &ArgMatches<'_>, conn_vars: &ConnVars) -> Result<Self, anyhow::Error> {
        let local_port;

        match matches.value_of(arg::SSH_TUNNEL) {
            Some(port) => local_port = port,
            None => return Err(anyhow!("Missing value for argument '{}'", arg::SSH_TUNNEL)),
        };

        let local_url = &(LOCALHOST.to_string() + ":" + local_port) as &str;
        let db_url = conn_vars.db_host.to_string() + ":" + &conn_vars.db_port;
        let port_fwd = local_url.to_string() + ":" + &db_url;
        let ssh_conn = conn_vars.db_user.to_string() + "@" + &conn_vars.db_host;
        let process = Command::new("ssh")
            .args(&["-N", "-T", "-L", &port_fwd, &ssh_conn])
            .spawn()?;
        let ssh_tunnel = SshTunnel {
            process,
            url: local_url.to_string(),
        };
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
                    println!("Closed ssh tunnel.");
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
}
