use serde::*;
use std::error::Error as ErrorTrait;
use std::fmt;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Error {
    details: String,
}

impl Error {
    pub fn new(msg: String) -> Self {
        Error { details: msg }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl ErrorTrait for Error {
    fn description(&self) -> &str {
        &self.details
    }
}
