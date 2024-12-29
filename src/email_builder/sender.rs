#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Sender<'a>(pub &'a str);

impl AsRef<str> for Sender<'_> {
    fn as_ref(&self) -> &str {
        self.0
    }
}
