use std::error::Error as StdError;
use std::fmt;

use tera::Error as TeraError;

#[derive(Debug)]
pub struct Error {
    kind: ErrorKind,
    source: Option<Box<dyn StdError + 'static>>,
}

#[derive(Debug)]
enum ErrorKind {
    ParseError,
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
            _ => write!(f, "Nonexhaustive"),
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
