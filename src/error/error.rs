use std::error::Error;
use std::fmt;

#[derive(Debug, PartialEq)]
pub struct MatrixError {
    details: String,
}

impl MatrixError {
    pub fn new(msg: &str) -> MatrixError {
        MatrixError {
            details: msg.to_string(),
        }
    }
}

impl fmt::Display for MatrixError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl Error for MatrixError {
    fn description(&self) -> &str {
        &self.details
    }
}
