use crate::{arg, email_handler::Email};
use anyhow::Result;
use clap::ArgMatches;
use rusoto_core::{HttpClient, Region};
use rusoto_credential::{EnvironmentProvider, ProvideAwsCredentials};
use rusoto_sesv2::{
    Body, Content as SesContent, Destination, EmailContent as SesEmailContent,
    Message as SesMessage, SendEmailRequest, SesV2, SesV2Client,
};

pub const CHARSET: &str = "UTF-8";

#[tokio::main]
async fn get_credentials(provider: &EnvironmentProvider) -> Result<(), anyhow::Error> {
    let _credentials = provider.credentials().await?;
    Ok(())
}

pub fn setup_ses_client(matches: &ArgMatches<'_>) -> Result<SesV2Client, anyhow::Error> {
    println!("Setting up email client...");
    let http = HttpClient::new()?;
    let provider = EnvironmentProvider::default();

    // Check if AWS access keys are set in environment
    if matches.is_present(arg::DRY_RUN) {
        get_credentials(&provider)?;
    }

    let client = SesV2Client::new_with(http, provider, Region::EuWest1);
    Ok(client)
}

#[tokio::main]
pub async fn send_email(email: &Email, client: &SesV2Client) -> Result<(), anyhow::Error> {
    let subject = &email.message.subject;
    let text = &email.message.text;
    let html = &email.message.html;
    let message = SesMessage {
        subject: SesContent {
            charset: Some(CHARSET.to_string()),
            data: subject.to_string(),
        },
        body: Body {
            text: text.as_ref().map(|text| SesContent {
                data: text.to_string(),
                charset: Some(CHARSET.to_string()),
            }),
            html: html.as_ref().map(|html| SesContent {
                data: html.to_string(),
                charset: Some(CHARSET.to_string()),
            }),
        },
    };
    let request = SendEmailRequest {
        from_email_address: Some(email.sender.to_string()),
        destination: Some(Destination {
            to_addresses: Some(vec![email.receiver.to_string()]),
            bcc_addresses: None,
            cc_addresses: None,
        }),
        content: SesEmailContent {
            raw: None,
            template: None,
            simple: Some(message),
        },
        configuration_set_name: None,
        email_tags: None,
        feedback_forwarding_email_address: None,
        reply_to_addresses: None,
        feedback_forwarding_email_address_identity_arn: None,
        from_email_address_identity_arn: None,
        list_management_options: None,
    };

    let _response = client.send_email(request).await?;

    Ok(())
}
