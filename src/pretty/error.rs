use std::{error, fmt, io, result};

#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    Fmt(fmt::Error),
}

pub type Result = result::Result<(), Error>;

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Self::Io(err)
    }
}

impl From<fmt::Error> for Error {
    fn from(err: fmt::Error) -> Self {
        Self::Fmt(err)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Io(err) => write!(f, "I/O Error: {}", err),
            Error::Fmt(err) => write!(f, "Formatting Error: {}", err),
        }
    }
}

impl error::Error for Error {}
