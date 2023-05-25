use super::method::Method;
use std::convert::{From, TryFrom};
use std::error::Error;
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};
use std::str::{from_utf8, Utf8Error};

pub struct Request {
    path: String,
    query: Option<String>,
    method: Method,
    protocol: String,
}

fn get_next_word(request: &str) -> Option<(&str, &str)> {
    for (i, c) in request.chars().enumerate() {
        if c == ' ' || c == '\r' {
            return Some((&request[..i], &request[i + 1..]));
        }
    }

    None
}

impl TryFrom<&[u8]> for Request {
    type Error = RequestParseError;

    fn try_from(buffer: &[u8]) -> Result<Self, Self::Error> {
        let request = from_utf8(buffer)?;

        let (method, request) = get_next_word(request).ok_or(RequestParseError::InvalidRequest)?;

        let (path, request) = get_next_word(request).ok_or(RequestParseError::InvalidRequest)?;

        let (protocol, _) = get_next_word(request).ok_or(RequestParseError::InvalidRequest)?;

        if protocol != "HTTP/1.1" {
            return Err(RequestParseError::InvalidProtocol);
        }

        unimplemented!()
    }
}

pub enum RequestParseError {
    InvalidRequest,
    InvalidEncoding,
    InvalidProtocol,
    InvalidMethod,
}

impl RequestParseError {
    fn message(&self) -> &str {
        match self {
            Self::InvalidRequest => "InvalidRequest",
            Self::InvalidEncoding => "InvalidEncoding",
            Self::InvalidProtocol => "InvalidProtocol",
            Self::InvalidMethod => "InvalidMethod",
        }
    }
}

impl From<Utf8Error> for RequestParseError {
    fn from(_: Utf8Error) -> Self {
        RequestParseError::InvalidEncoding
    }
}

impl Display for RequestParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl Debug for RequestParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl Error for RequestParseError {}
