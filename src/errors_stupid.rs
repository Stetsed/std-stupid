use core::fmt;
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::str::Utf8Error;

#[derive(Debug)]
pub struct intValueError {
    pub source: String,
}

impl intValueError {
    pub fn new<T: Into<String>>(source: T) -> Self {
        intValueError {
            source: source.into(),
        }
    }
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

impl HttpServerError {
    pub fn new<T: Into<String>>(source: T) -> Self {
        HttpServerError {
            source: source.into(),
        }
    }
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

impl subStringError {
    pub fn new<T: Into<String>>(source: T) -> Self {
        subStringError {
            source: source.into(),
        }
    }
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum HttpReturnError {
    HttpServerError(crate::HttpServerError),
    IntValueError(crate::intValueError),
    SubStringError(crate::subStringError),
    Utf8ParsingError(std::str::Utf8Error),
}

impl Error for HttpReturnError {}

impl Display for HttpReturnError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Httpreturn")
    }
}
