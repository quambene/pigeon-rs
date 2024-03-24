use super::{SendEmail, SentEmail, Status};
use crate::email_builder::Email;
use clap::ArgMatches;

pub struct MockClient;

impl<'a> SendEmail<'a> for MockClient {
    fn send(
        &self,
        _matches: &ArgMatches,
        email: &'a Email<'a>,
    ) -> Result<SentEmail<'a>, anyhow::Error> {
        let email = SentEmail::new(email, Status::DryRun);
        Ok(email)
    }
}
