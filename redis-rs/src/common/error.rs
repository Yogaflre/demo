#[derive(Debug)]
pub enum ErrorKind {
    Parser,
    Invalid,
}

#[derive(Debug)]
pub struct Error {
    kind: ErrorKind,
    details: Box<[u8]>,
}
impl Error {
    pub fn new(kind: ErrorKind, details: &str) -> Self {
        return Self {
            kind,
            details: details.as_bytes().into(),
        };
    }
}
