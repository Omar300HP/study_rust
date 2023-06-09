use super::method::{Method, MethodError};
use std::convert::{From, TryFrom};
use std::error::Error;
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};
use std::str::{from_utf8, Utf8Error};

pub struct Request<'buf> {
    path: &'buf str,
    query: Option<&'buf str>,
    method: Method,
    protocol: &'buf str,
}

fn get_next_word(request: &str) -> Option<(&str, &str)> {
    for (i, c) in request.chars().enumerate() {
        if c == ' ' || c == '\r' {
            return Some((&request[..i], &request[i + 1..]));
        }
    }

    None
}

impl<'buf> TryFrom<&'buf [u8]> for Request<'buf> {
    type Error = RequestParseError;

    fn try_from(buffer: &'buf [u8]) -> Result<Request<'buf>, Self::Error> {
        let request = from_utf8(buffer)?;

        let (method, request) = get_next_word(request).ok_or(RequestParseError::InvalidRequest)?;

        let (mut path, request) =
            get_next_word(request).ok_or(RequestParseError::InvalidRequest)?;

        let (protocol, _) = get_next_word(request).ok_or(RequestParseError::InvalidRequest)?;

        if protocol != "HTTP/1.1" {
            return Err(RequestParseError::InvalidProtocol);
        }

        let method: Method = method.parse()?;

        let mut query = None;

        if let Some(index) = path.find("?") {
            query = Some(&path[index + 1..]);
            path = &path[..index];
        }

        Ok(Self {
            path,
            query,
            method,
            protocol,
        })
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

impl From<MethodError> for RequestParseError {
    fn from(_: MethodError) -> Self {
        RequestParseError::InvalidMethod
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
