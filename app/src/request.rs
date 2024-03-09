use std::fmt;
use reqwest::header::HeaderMap;
use serde::{Serialize, de::DeserializeOwned};
use anyhow::Result;
use futures::Future;

use crate::auth::Auth;

#[derive(Debug, Clone, Copy)]
pub enum RequestMethod {
    GET,
    POST,
}

impl fmt::Display for RequestMethod {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RequestMethod::GET => write!(f, "GET"),
            RequestMethod::POST => write!(f, "POST"),
        }
    }
}

pub trait Request: Auth {
    fn build_headers<T: Serialize>(&self, timestamp: u64, method: RequestMethod, path: &str, parameters: Option<T>) -> Result<HeaderMap>;
    fn send_request<T: Serialize + Clone>(&self, method: RequestMethod, path: &str, parameters: Option<T>, access_level: AccessLevel) -> impl Future<Output = Result<String>>;
    fn deserialzie_response<P: DeserializeOwned>(&self, body: &str) -> Result<P>;
}

pub enum AccessLevel {
    Public,
    Private,
}

impl fmt::Display for AccessLevel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AccessLevel::Public => write!(f, "/public"),
            AccessLevel::Private => write!(f, "/private"),
        }
    }
}