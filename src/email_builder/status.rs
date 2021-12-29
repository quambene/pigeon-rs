#[derive(Debug, Clone)]
pub enum Status {
    NotSent,
    DryRun,
    Sent(String),
    SentError(String),
}
