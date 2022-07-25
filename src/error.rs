use std::fmt;

#[derive(Debug)]
pub enum Error {
    FileNotFound,
    UnableToParse(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::FileNotFound => write!(f, "File not found"),
            Error::UnableToParse(reason) => write!(f, "Unable to parse, reason: {}", reason),
        }
    }
}

pub type Result<T> = std::result::Result<T, Error>;
