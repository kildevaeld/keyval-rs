use std::error::Error as StdError;
use std::fmt;

#[derive(Debug)]
pub enum Error {
    NotFound,
    Expired,
    Backend(Box<dyn StdError + Send>),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Backend(b) => write!(f, "backend error: {}", b),
            Error::NotFound => write!(f, "not found error"),
            Error::Expired => write!(f, "expired error"),
        }
    }
}

impl StdError for Error {}
