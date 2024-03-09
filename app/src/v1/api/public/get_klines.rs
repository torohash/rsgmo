use crate::{
    v1::api::GmoApi,
    request::AccessLevel,
    utils,
};

use anyhow::Result;
use serde::{Deserialize, Serialize};

const PATH: &str = "/v1/klines";

impl GmoApi {
    pub async fn get_klines(&self, parameters: Option<GetKlinesParameters>) -> Result<GetKlinesResponse> {
        self.get(PATH, parameters, AccessLevel::Public).await
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct GetKlinesParameters {
    symbol: String,
    interval: String,
    date: String,
}

impl GetKlinesParameters {
    pub fn new(symbol: &str, interval: &str, date: &str) -> Self {
        GetKlinesParameters {
            symbol: symbol.to_string(),
            interval: interval.to_string(),
            date: date.to_string(),
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct GetKlinesResponse {
    status: i32,
    data: Vec<KlinesData>,
    responsetime: String,
}

impl GetKlinesResponse {
    pub fn status(&self) -> i32 {
        self.status
    }
    pub fn data(&self) -> &Vec<KlinesData> {
        &self.data
    }
    pub fn responsetime(&self) -> &str {
        &self.responsetime
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KlinesData {
    #[serde(deserialize_with = "utils::deserialize_string_to_u64")]
    open_time: u64,
    #[serde(deserialize_with = "utils::deserialize_string_to_u64")]
    open: u64,
    #[serde(deserialize_with = "utils::deserialize_string_to_u64")]
    high: u64,
    #[serde(deserialize_with = "utils::deserialize_string_to_u64")]
    low: u64,
    #[serde(deserialize_with = "utils::deserialize_string_to_u64")]
    close: u64,
    #[serde(deserialize_with = "utils::deserialize_f64")]
    volume: f64,
}

impl KlinesData {
    pub fn open_time(&self) -> u64 {
        self.open_time
    }
    pub fn open(&self) -> u64 {
        self.open
    }
    pub fn high(&self) -> u64 {
        self.high
    }
    pub fn low(&self) -> u64 {
        self.low
    }
    pub fn close(&self) -> u64 {
        self.close
    }
    pub fn volume(&self) -> f64 {
        self.volume
    }

}