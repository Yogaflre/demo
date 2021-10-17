use std::fmt::Display;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum ErrorKind {
    Parser,
    Invalid,
    IO,
}

#[derive(Debug)]
pub struct Error {
    kind: ErrorKind,
    details: String,
}

impl Error {
    pub fn new(kind: ErrorKind, details: String) -> Self {
        return Self { kind, details };
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(f, "{}", self);
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        return Error::new(ErrorKind::IO, e.to_string());
    }
}

impl From<bincode::Error> for Error {
    fn from(e: bincode::Error) -> Self {
        return Error::new(ErrorKind::Parser, e.to_string());
    }
}
