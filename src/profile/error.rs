use std::error::Error as StdError;
use std::fmt::{self, Display};

#[derive(Debug, Copy, Clone)]
pub enum Error {
    RunException,       // Error with running firefox
    DirectoryException, // Error making or getting a user directory
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::RunException => write!(f, "Unknown error trying to run firefox"),
            Error::DirectoryException => {
                write!(f, "Error while trying to read firefox profiles directory")
            }
        }
    }
}

impl StdError for Error {}
