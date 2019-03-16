use std::error::Error as StdError;
use std::fmt;
use std::io::Error as IOError;

use git2::Error as Git2Error;

use tera::Error as TeraError;

#[derive(Debug)]
pub struct Error {
    kind: ErrorKind,
    source: Option<Box<dyn StdError + 'static>>,
}

#[derive(Debug)]
enum ErrorKind {
    ParseError,
    GitError,
    IOError,
}

impl StdError for Error {
    fn cause(&self) -> Option<&(dyn StdError)> {
        self.source().as_ref().map(|c| &**c)
    }

    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        self.source
            .as_ref()
            .map(|c| c.as_ref() as &(dyn StdError + 'static))
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.kind {
            ErrorKind::ParseError => write!(f, "Error parsing template"),
            ErrorKind::GitError => write!(f, "Error initiating git repo"),
            ErrorKind::IOError => write!(f, "io error"),
        }
    }
}

impl From<TeraError> for Error {
    fn from(error: TeraError) -> Error {
        Error {
            kind: ErrorKind::ParseError,
            source: Some(error.into()),
        }
    }
}

impl From<Git2Error> for Error {
    fn from(error: Git2Error) -> Error {
        Error {
            kind: ErrorKind::GitError,
            source: Some(error.into()),
        }
    }
}

impl From<IOError> for Error {
    fn from(error: IOError) -> Error {
        Error {
            kind: ErrorKind::IOError,
            source: Some(error.into()),
        }
    }
}
