use core::fmt;
use std::error::Error;
use std::fmt::{Display, Formatter};

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

#[derive(Debug)]
pub struct HttpServerError {
    pub source: String,
}

impl Error for HttpServerError {}

impl Display for HttpServerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "HttpServerError")
    }
}

#[derive(Debug)]
pub struct subStringError {
    pub source: String,
}

impl Error for subStringError {}

impl Display for subStringError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "HttpServerError")
    }
}
