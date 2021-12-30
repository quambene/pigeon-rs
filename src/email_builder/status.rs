#[derive(Debug, Clone)]
pub enum Status {
    DryRun,
    SentOk(String),
    SentError(String),
}
