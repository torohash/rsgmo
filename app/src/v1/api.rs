pub mod private;
pub mod public;

use crate::{
    auth::Auth, constants::API_BASE_URL, request::{AccessLevel, Request, RequestMethod}, utils
};
use anyhow::{anyhow, Result};
use ring::hmac;
use serde::{de::DeserializeOwned, Serialize, Deserialize};
use reqwest::header::{HeaderMap, HeaderValue};
use serde_json::Value;

#[derive(Debug, Clone)]
pub struct GmoApi {
    base_url: String,
    api_key: Option<String>,
    api_secret: Option<String>,
}

impl GmoApi {
    pub fn new(api_key: Option<String>, api_secret: Option<String>) -> Self {
        GmoApi {
            base_url: API_BASE_URL.to_string(),
            api_key,
            api_secret,
        }
    }
    pub fn base_url(&self) -> &str {
        &self.base_url
    }
    pub fn api_key(&self) -> &Option<String> {
        &self.api_key
    }
    pub fn api_secret(&self) -> &Option<String> {
        &self.api_secret
    }

    async fn get<T: Serialize + Clone, P: DeserializeOwned>(&self, path: &str, parameters: Option<T>, access_level: AccessLevel) -> Result<P> {
        let body = self.send_request(RequestMethod::GET, path, parameters, access_level).await?;
        self.deserialize_response(&body)
    }

    async fn post<T: Serialize + Clone, P: DeserializeOwned>(&self, path: &str, parameters: Option<T>, access_level: AccessLevel) -> Result<P> {
        let body = self.send_request(RequestMethod::POST, path, parameters, access_level).await?;
        self.deserialize_response(&body)
    }
}

impl Auth for GmoApi {
    fn require_api_key(&self) -> Result<&str> {
        match &self.api_key {
            Some(key) => Ok(key),
            None => Err(anyhow::anyhow!("api_key is required")),
        }
    }
    fn require_api_secret(&self) -> Result<&str> {
        match &self.api_secret {
            Some(secret) => Ok(secret),
            None => Err(anyhow::anyhow!("api_secret is required")),
        }
    }
    fn create_signature<T: Serialize>(&self, timestamp: u64, method: RequestMethod, path: &str, parameters: Option<T>) -> Result<String> {
        let api_secret = self.require_api_secret()?;

        let message = match method {
            RequestMethod::GET => format!("{}{}{}", timestamp, method, path),
            RequestMethod::POST => {
                let parameters_json = serde_json::to_string(&parameters)?;
                format!("{}{}{}{}", timestamp, method, path, parameters_json)
            },
        };
        let signed_key = hmac::Key::new(hmac::HMAC_SHA256, api_secret.as_bytes());
        let signature = hex::encode(hmac::sign(&signed_key, message.as_bytes()).as_ref());

        Ok(signature)
    }
}

impl Request for GmoApi {
    fn build_headers<T: Serialize>(&self, timestamp: u64, method: RequestMethod, path: &str, parameters: Option<T>) -> Result<HeaderMap> {
        let mut headers = HeaderMap::new();
        headers.insert("API-KEY", HeaderValue::from_str(self.require_api_key()?)?);
        headers.insert("API-TIMESTAMP", HeaderValue::from_str(&timestamp.to_string())?);
        headers.insert("API-SIGN", HeaderValue::from_str(&self.create_signature(timestamp, method, path, parameters)?)?);
        headers.insert("Content-Type", HeaderValue::from_static("application/json"));

        Ok(headers)
    }
    async fn send_request<T: Serialize + Clone>(&self, method: RequestMethod, path: &str, parameters: Option<T>, access_level: AccessLevel) -> Result<String> {
        let url = format!("{}{}{}", self.base_url(), access_level, path);
        let timestamp = utils::get_timestamp()?;

        let client = reqwest::Client::new();
        let response = match method {
            RequestMethod::GET => {
                match access_level {
                    AccessLevel::Public => {
                        match parameters {
                            Some(parameters) => client.get(&url).query(&parameters).send().await?,
                            None => client.get(&url).send().await?,
                        }
                    },
                    AccessLevel::Private => {
                        let headers = self.build_headers(timestamp, method, path, parameters.clone())?;
                        match parameters {
                            Some(parameters) => client.get(&url).headers(headers).query(&parameters).send().await?,
                            None => client.get(&url).headers(headers).send().await?,
                        }
                    },
                }
            },
            RequestMethod::POST => {
                let headers = self.build_headers(timestamp, method, path, parameters.clone())?;
                match parameters {
                    Some(parameters) => client.post(&url).headers(headers).json(&parameters).send().await?,
                    None => client.post(&url).headers(headers).send().await?,
                }
            },
        };

        match response.status().as_u16() {
            200 => {
                let body = response.text().await?;
                Ok(body)
            },
            _ => {
                Err(anyhow!(format!(
                    "Request error status code {}: response details: {:?}",
                    response.status().as_u16(),
                    response.text().await?
                )))
            },
        }
    }
    fn deserialize_response<P: serde::de::DeserializeOwned>(&self, body: &str) -> Result<P> {
        let response: Value = serde_json::from_str(body)?;

        let status = response.get("status").and_then(Value::as_i64);
        match status {
            Some(status) => {
                match status {
                    0 => {
                        let response: P = serde_json::from_str(body)?;
                        Ok(response)
                    },
                    _ => Err(anyhow!("Response error: {}", response))
                }
            },
            None => {
                Err(anyhow!("Response error {}", response))
            },
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Pagination {
    current_page: i32,
    count: i32,
}

impl Pagination {
    pub fn current_page(&self) -> i32 {
        self.current_page
    }
    pub fn count(&self) -> i32 {
        self.count
    }
}