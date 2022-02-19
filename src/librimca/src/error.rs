use std::{error::Error as StdError, fmt, io};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub enum Error {
    LaunchError(String),
    IoError(String),
    ApiError(String),
    StateError(String),
	InstanceDoesNotExist
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Error::LaunchError(ref e) => f.write_str(e),
            Error::IoError(ref e) => f.write_str(e),
            Error::ApiError(ref e) => f.write_str(e),
            Error::StateError(ref e) => f.write_str(e),
            Error::InstanceDoesNotExist => f.write_str("instance does not exist"),
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

impl From<LaunchError> for Error {
    fn from(e: LaunchError) -> Self {
        Error::LaunchError(e.to_string())
    }
}

impl From<StateError> for Error {
    fn from(e: StateError) -> Self {
        Error::StateError(e.to_string())
    }
}



#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub enum ApiError {
	SerdeError(String),
	ReqwestError(String),
	CannotFindLatestVersion,
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            ApiError::CannotFindLatestVersion => f.write_str("cannot find latest version"),
            ApiError::SerdeError(ref e) => f.write_str(e),
            ApiError::ReqwestError(ref e) => f.write_str(e),
        }
    }
}

impl From<serde_json::Error> for ApiError {
   fn from(e: serde_json::Error) -> Self {
        ApiError::SerdeError(e.to_string())
   }	
}

impl From<reqwest::Error> for ApiError {
   fn from(e: reqwest::Error) -> Self {
        ApiError::ReqwestError(e.to_string())
   }
}



#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub enum LaunchError {
	Temp
}

impl fmt::Display for LaunchError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            LaunchError::Temp => f.write_str("temp"),
        }
    }
}





#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub enum StateError {
	IoError(String),
	SerdeError(String),
}

impl fmt::Display for StateError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            StateError::IoError(ref e) => f.write_str(e),
            StateError::SerdeError(ref e) => f.write_str(e),
        }
    }
}

impl From<io::Error> for StateError {
    fn from(e: io::Error) -> Self {
        StateError::IoError(e.to_string())
    }
}

impl From<serde_json::Error> for StateError {
   fn from(e: serde_json::Error) -> Self {
        StateError::SerdeError(e.to_string())
   }	
}