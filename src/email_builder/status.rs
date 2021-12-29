#[derive(Debug, Clone)]
pub enum Status {
    DryRun,
    Sent(String),
    SentError(String),
}
