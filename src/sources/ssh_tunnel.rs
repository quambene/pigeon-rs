use crate::sources::postgres::ConnVars;
use anyhow::{anyhow, Context, Result};
use std::{
    env,
    process::{Child, Command},
};
use url::Url;

pub struct SshTunnel {
    process: Child,
    connection_url: Url,
}

const LOCALHOST: &str = "127.0.0.1";

impl SshTunnel {
    pub fn new(ssh_tunnel: &str, conn_vars: &ConnVars) -> Result<Self, anyhow::Error> {
        println!("Opening ssh tunnel ...");

        let server_host = env::var("SERVER_HOST")
            .context("Missing environment variable 'SERVER_HOST'. Needed for ssh tunnel.")?;
        let server_user = env::var("SERVER_USER")
            .context("Missing environment variable 'SERVER_USER'. Needed for ssh tunnel.")?;

        let local_port = ssh_tunnel;
        let local_url = &(LOCALHOST.to_string() + ":" + local_port) as &str;
        let db_url = format!("{}:{}", conn_vars.db_host, &conn_vars.db_port);

        let port_fwd = format!("{}:{}", local_url, &db_url);
        let ssh_connection = format!("{}@{}", server_user, &server_host);

        let process = Command::new("ssh")
            .args(["-N", "-T", "-L", &port_fwd, &ssh_connection])
            .spawn()?;

        let connection_url = format!(
            "postgresql://{}:{}@{}/{}",
            &conn_vars.db_user, &conn_vars.db_password.0, local_url, &conn_vars.db_name
        );
        let connection_url = Url::parse(&connection_url)?;

        let ssh_tunnel = SshTunnel {
            process,
            connection_url,
        };

        println!(
            "Ssh tunnel openend: forwarding port from {} to {} for user '{}' on server '{}'",
            local_url, db_url, server_user, server_host
        );
        Ok(ssh_tunnel)
    }

    pub fn connection_url(&self) -> &Url {
        &self.connection_url
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
}
