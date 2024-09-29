use core::fmt;
use std::error::Error;
use std::fmt::Display;
use std::num::ParseFloatError;
use std::str::Utf8Error;

#[derive(Debug)]
pub struct IntValueError {
    pub source: String,
}

impl IntValueError {
    pub fn new<T: Into<String>>(source: T) -> Self {
        IntValueError {
            source: source.into(),
        }
    }
}

impl Error for IntValueError {}

impl Display for IntValueError {
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
pub struct SubStringError {
    pub source: String,
}

impl Error for SubStringError {}

impl Display for SubStringError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "HttpServerError")
    }
}

impl SubStringError {
    pub fn new<T: Into<String>>(source: T) -> Self {
        SubStringError {
            source: source.into(),
        }
    }
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum StdStupidError {
    HttpServer(HttpServerError),
    IntValue(IntValueError),
    SubString(SubStringError),
    Utf8Parsing(std::str::Utf8Error),
    ParseFloat(std::num::ParseFloatError),
    StdIO(std::io::Error),
    AddrParse(std::net::AddrParseError),
    From(),
}

impl Error for StdStupidError {}

impl Display for StdStupidError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "stdStupidError")
    }
}

impl From<HttpServerError> for StdStupidError {
    fn from(error: HttpServerError) -> Self {
        Self::HttpServer(error)
    }
}

impl From<IntValueError> for StdStupidError {
    fn from(error: IntValueError) -> Self {
        Self::IntValue(error)
    }
}

impl From<SubStringError> for StdStupidError {
    fn from(error: SubStringError) -> Self {
        Self::SubString(error)
    }
}

impl From<Utf8Error> for StdStupidError {
    fn from(error: Utf8Error) -> Self {
        Self::Utf8Parsing(error)
    }
}

impl From<ParseFloatError> for StdStupidError {
    fn from(error: ParseFloatError) -> Self {
        Self::ParseFloat(error)
    }
}

impl From<std::io::Error> for StdStupidError {
    fn from(error: std::io::Error) -> Self {
        Self::StdIO(error)
    }
}

impl From<std::net::AddrParseError> for StdStupidError {
    fn from(error: std::net::AddrParseError) -> Self {
        Self::AddrParse(error)
    }
}
