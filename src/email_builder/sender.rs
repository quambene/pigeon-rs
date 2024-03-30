#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Sender<'a>(pub &'a str);

impl<'a> AsRef<str> for Sender<'a> {
    fn as_ref(&self) -> &str {
        self.0
    }
}
