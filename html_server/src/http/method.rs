use std::error::Error;
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};
use std::str::FromStr;

pub enum Method {
    GET,
    POST,
    DELETE,
    PUT,
    PATCH,
    OPTIONS,
    HEAD,
    CONNECT,
    TRACE,
}

impl FromStr for Method {
    type Err = MethodError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "GET" => Ok(Self::GET),
            "POST" => Ok(Self::POST),
            "DELETE" => Ok(Self::DELETE),
            "PUT" => Ok(Self::PUT),
            "PATCH" => Ok(Self::PATCH),
            "OPTIONS" => Ok(Self::OPTIONS),
            "HEAD" => Ok(Self::HEAD),
            "CONNECT" => Ok(Self::CONNECT),
            "TRACE" => Ok(Self::TRACE),
            _ => Err(MethodError::create()),
        }
    }
}

pub struct MethodError {}

impl MethodError {
    fn create() -> Self {
        Self {}
    }
}

impl Display for MethodError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", "Invalid HTTP Method")
    }
}
impl Debug for MethodError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", "Invalid HTTP Method")
    }
}

impl Error for MethodError {}
