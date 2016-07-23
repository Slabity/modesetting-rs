use errno::Errno;

use std::fmt;
use std::error::Error as StdError;
use std::result::Result as StdResult;
use std::io::Error as IoError;
use std::io::Write;
use std::convert::From;

#[derive(Debug)]
pub enum Error {
    Io(IoError),
    Ioctl(Errno),
    NotAvailable
}

pub type Result<T> = StdResult<T, Error>;

impl fmt::Display for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Io(ref err) => err.fmt(fmt),
            Error::Ioctl(ref err) => err.fmt(fmt),
            _ => write!(fmt, "Not available")
        }
    }
}

impl StdError for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Io(ref err) => err.description(),
            _ => ""
        }
    }

    fn cause(&self) -> Option<&StdError> {
        match *self {
            Error::Io(ref err) => err.cause(),
            _ => None
        }
    }
}

impl From<IoError> for Error {
    fn from(err: IoError) -> Error {
        Error::Io(err)
    }
}

impl From<Errno> for Error {
    fn from(err: Errno) -> Error {
        Error::Ioctl(err)
    }
}
