use core::fmt;
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::str::Utf8Error;

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

#[derive(Debug)]
pub enum httpReturnError {
    httpServerError(HttpServerError),
    subStringError(subStringError),
    Utf8ParsingError(Utf8Error),
}

impl Error for httpReturnError {}

impl Display for httpReturnError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Httpreturn")
    }
}
