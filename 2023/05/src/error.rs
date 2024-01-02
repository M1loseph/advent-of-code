use core::fmt;
use std::num::ParseIntError;

#[derive(Debug)]
pub struct AlmanacError {
    pub message: String,
}

impl From<ParseIntError> for AlmanacError {
    fn from(err: ParseIntError) -> Self {
        AlmanacError {
            message: err.to_string(),
        }
    }
}

impl From<std::io::Error> for AlmanacError {
    fn from(err: std::io::Error) -> Self {
        AlmanacError {
            message: err.to_string(),
        }
    }
}

impl fmt::Display for AlmanacError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "AlmanacError: {}", self.message)
    }
}
