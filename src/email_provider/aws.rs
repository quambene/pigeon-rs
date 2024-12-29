use crate::{
    arg,
    email_builder::Email,
    email_transmission::{SendEmail, SentEmail, Status},
    utils::format_green,
};
use anyhow::{Context, Result};
use base64::{engine::general_purpose::STANDARD as BASE64, Engine};
use bytes::Bytes;
use clap::ArgMatches;
use rusoto_core::{HttpClient, Region};
use rusoto_credential::{EnvironmentProvider, ProvideAwsCredentials};
use rusoto_ses::{RawMessage, SendRawEmailRequest, Ses, SesClient};
use std::{env, str::FromStr};

pub struct AwsSesClient {
    #[allow(dead_code)]
    pub region_name: String,
    pub client: SesClient,
}

impl AwsSesClient {
    pub fn new(matches: &ArgMatches) -> Result<Self, anyhow::Error> {
        let http = HttpClient::new()?;
        let provider = EnvironmentProvider::default();

        let aws_region =
            env::var("AWS_REGION").context("Missing environment variable 'AWS_REGION'")?;
        let region = Region::from_str(&aws_region).context("Unknown aws region")?;
        let region_name = region.name().to_string();

        // Check if AWS access keys are set in environment
        if matches.get_flag(arg::DRY_RUN) {
            AwsSesClient::get_credentials(&provider).context(
                "Missing environment variable 'AWS_ACCESS_KEY_ID' and/or 'AWS_SECRET_ACCESS_KEY'",
            )?;
        }

        let client = SesClient::new_with(http, provider, region);

        println!(
            "Connecting to aws server in region '{}' ... {}",
            region_name,
            format_green("ok")
        );

        Ok(AwsSesClient {
            region_name,
            client,
        })
    }

    #[tokio::main]
    async fn get_credentials(provider: &EnvironmentProvider) -> Result<(), anyhow::Error> {
        let _credentials = provider.credentials().await?;
        Ok(())
    }
}

impl<'a> SendEmail<'a> for AwsSesClient {
    #[tokio::main]
    async fn send(&self, email: &'a Email<'a>) -> Result<SentEmail<'a>, anyhow::Error> {
        let raw_message = RawMessage {
            data: Bytes::from(BASE64.encode(email.mime_format.message.formatted())),
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
        let sent_email = SentEmail::new(email, status);

        Ok(sent_email)
    }
}
