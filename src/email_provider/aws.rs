use crate::{
    arg,
    email_builder::Email,
    email_transmission::{SendEmail, SentEmail, Status},
    helper::format_green,
};
use anyhow::Result;
use bytes::Bytes;
use clap::ArgMatches;
use rusoto_core::{HttpClient, Region};
use rusoto_credential::{EnvironmentProvider, ProvideAwsCredentials};
use rusoto_ses::{RawMessage, SendRawEmailRequest, Ses, SesClient};

pub struct AwsSesClient {
    pub region_name: String,
    pub client: SesClient,
}

impl AwsSesClient {
    pub fn new(matches: &ArgMatches) -> Result<Self, anyhow::Error> {
        let http = HttpClient::new()?;
        let provider = EnvironmentProvider::default();
        let region = Region::EuWest1;
        let region_name = region.name().to_string();

        // Check if AWS access keys are set in environment
        if matches.is_present(arg::DRY_RUN) {
            get_credentials(&provider)?;
        }

        let client = SesClient::new_with(http, provider, region);

        Ok(AwsSesClient {
            region_name,
            client,
        })
    }

    pub fn display_connection_status(&self, connection: &str) {
        println!(
            "Connected to {} server in region '{}' ... {}",
            connection,
            self.region_name,
            format_green("ok")
        );
    }
}

impl<'a> SendEmail<'a> for AwsSesClient {
    #[tokio::main]
    async fn send(
        &self,
        matches: &ArgMatches,
        email: &'a Email<'a>,
    ) -> Result<SentEmail<'a>, anyhow::Error> {
        let sent_email = if matches.is_present(arg::DRY_RUN) {
            let status = Status::DryRun;
            SentEmail::new(email, status)
        } else {
            let raw_message = RawMessage {
                data: Bytes::from(email.mime_format.message.formatted()),
            };
            let request = SendRawEmailRequest {
                raw_message,
                ..Default::default()
            };
            let response = self.client.send_raw_email(request).await;
            let status = match response {
                Ok(response) => Status::SentOk(response.message_id),
                Err(err) => Status::SentError(err.to_string()),
            };
            SentEmail::new(email, status)
        };

        Ok(sent_email)
    }
}

#[tokio::main]
async fn get_credentials(provider: &EnvironmentProvider) -> Result<(), anyhow::Error> {
    let _credentials = provider.credentials().await?;
    Ok(())
}
