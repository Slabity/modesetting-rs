use std::fmt;
use std::error::Error as StdError;
use std::result::Result as StdResult;
use std::io::Error as IoError;
use std::convert::From;

#[derive(Debug)]
pub enum Error {
    Io(IoError)
}

pub type Result<T> = StdResult<T, Error>;

impl fmt::Display for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Io(ref err) => err.fmt(fmt),
            //_ => Ok(())
        }
    }
}

impl StdError for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Io(ref err) => err.description(),
            //_ => ""
        }
    }

    fn cause(&self) -> Option<&StdError> {
        match *self {
            Error::Io(ref err) => err.cause(),
            //_ => None
        }
    }
}

impl From<IoError> for Error {
    fn from(err: IoError) -> Error {
        Error::Io(err)
    }
}