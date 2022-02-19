use std::{error::Error as StdError, fmt, io};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub enum Error {
    LaunchError,
    IoError(String),
    ApiError(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Error::LaunchError => f.write_str("launch error"),
            Error::IoError(ref e) => f.write_str(e),
            Error::ApiError(ref e) => f.write_str(e),
        }
    }
}

impl StdError for Error {}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Error::IoError(e.to_string())
    }
}

impl From<ApiError> for Error {
    fn from(e: ApiError) -> Self {
        Error::ApiError(e.to_string())
    }
}





#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub enum ApiError {
	CannotFindLatestVersion,
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            ApiError::CannotFindLatestVersion => f.write_str("cannot find latest version"),
        }
    }
}