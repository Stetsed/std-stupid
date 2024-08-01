use core::fmt;
use std::error::Error;
use std::fmt::Display;

pub mod errors_stupid {}

#[derive(Debug)]
pub struct intValueError {
    pub source: String,
}

impl Error for intValueError {}

impl Display for intValueError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "intValueError")
    }
}
