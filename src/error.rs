use std::error;
use std::fmt;
use std::io;
use std::path::PathBuf;

#[derive(Debug)]
pub enum Error {
    PeParsingError(PathBuf, Box<error::Error>),
    IoError(PathBuf, io::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::PeParsingError(p, e) => write!(
                f,
                "An error was encountered while reading the version fields of \"{}\": {}",
                p.display(),
                e
            ),
            Error::IoError(p, e) => write!(
                f,
                "An error was encountered while accessing the path \"{}\": {}",
                p.display(),
                e
            ),
        }
    }
}

impl error::Error for Error {
    fn cause(&self) -> Option<&error::Error> {
        match self {
            Error::PeParsingError(_, e) => Some(e.as_ref()),
            Error::IoError(_, e) => Some(e),
        }
    }
}
