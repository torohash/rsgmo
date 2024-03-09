use crate::{
    v1::api::GmoApi,
    request::AccessLevel,
    utils,
};

use anyhow::Result;
use serde::{Deserialize, Serialize};

const PATH: &str = "/v1/ticker";

impl GmoApi {
    pub async fn get_ticker(&self, parameters: Option<GetTickerParameters>) -> Result<GetTickerResponse> {
        self.get(PATH, parameters, AccessLevel::Public).await
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct GetTickerParameters {
    symbol: String,
}

impl GetTickerParameters {
    pub fn new(symbol: &str) -> Self {
        GetTickerParameters {
            symbol: symbol.to_string(),
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct GetTickerResponse {
    status: i32,
    data: Vec<TickerData>,
    responsetime: String,
}

impl GetTickerResponse {
    pub fn status(&self) -> i32 {
        self.status
    }
    pub fn data(&self) -> &Vec<TickerData> {
        &self.data
    }
    pub fn responsetime(&self) -> &str {
        &self.responsetime
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TickerData {
    #[serde(deserialize_with = "utils::deserialize_string_to_u64")]
    ask: u64,
    #[serde(deserialize_with = "utils::deserialize_string_to_u64")]
    bid: u64,
    #[serde(deserialize_with = "utils::deserialize_string_to_u64")]
    high: u64,
    #[serde(deserialize_with = "utils::deserialize_string_to_u64")]
    last: u64,
    #[serde(deserialize_with = "utils::deserialize_string_to_u64")]
    low: u64,
    symbol: String,
    timestamp: String,
    #[serde(deserialize_with = "utils::deserialize_f64")]
    volume: f64,
}

impl TickerData {
    pub fn ask(&self) -> u64 {
        self.ask
    }
    pub fn bid(&self) -> u64 {
        self.bid
    }
    pub fn high(&self) -> u64 {
        self.high
    }
    pub fn last(&self) -> u64 {
        self.last
    }
    pub fn low(&self) -> u64 {
        self.low
    }
    pub fn symbol(&self) -> &str {
        &self.symbol
    }
    pub fn timestamp(&self) -> &str {
        &self.timestamp
    }
    pub fn volume(&self) -> f64 {
        self.volume
    }
}