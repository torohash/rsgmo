use anyhow::Result;
use serde::Serialize;

use crate::request::RequestMethod;

pub trait Auth {
    fn require_api_key(&self) -> Result<&str>;
    fn require_api_secret(&self) -> Result<&str>;
    fn create_signature<T: Serialize>(&self, timestamp: u64, method: RequestMethod, path: &str, parameters: Option<T>) -> Result<String>;
}