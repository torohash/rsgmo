use crate::{
    v1::api::{GmoApi, Pagination},
    request::AccessLevel,
    utils,
};

use anyhow::Result;
use serde::{Deserialize, Serialize};

const PATH: &str = "/v1/trades";

impl GmoApi {
    pub async fn get_trades(&self, parameters: Option<GetTradesParameters>) -> Result<GetTradesResponse> {
        self.get(PATH, parameters, AccessLevel::Public).await
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct GetTradesParameters {
    symbol: String,
    page: Option<i32>,
    count: Option<i32>,
}

impl GetTradesParameters {
    pub fn new(symbol: &str) -> Self {
        GetTradesParameters {
            symbol: symbol.to_string(),
            page: None,
            count: None,
        }
    }
    pub fn with_page(mut self, page: i32) -> Self {
        self.page = Some(page);
        self
    }

    pub fn with_count(mut self, count: i32) -> Self {
        self.count = Some(count);
        self
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct GetTradesResponse {
    status: i32,
    data: TradesData,
    responsetime: String,
}

impl GetTradesResponse {
    pub fn status(&self) -> i32 {
        self.status
    }
    pub fn data(&self) -> &TradesData {
        &self.data
    }
    pub fn responsetime(&self) -> &str {
        &self.responsetime
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TradesData {
    pagination: Pagination,
    list: Vec<Trade>
}

impl TradesData {
    pub fn pagination(&self) -> &Pagination {
        &self.pagination
    }
    pub fn list(&self) -> &Vec<Trade> {
        &self.list
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Trade {
    side: String,
    #[serde(deserialize_with = "utils::deserialize_string_to_u64")]
    price: u64,
    #[serde(deserialize_with = "utils::deserialize_f64")]
    size: f64,
    timestamp: String,
}

impl Trade {
    pub fn side(&self) -> &str {
        &self.side
    }
    pub fn price(&self) -> u64 {
        self.price
    }
    pub fn size(&self) -> f64 {
        self.size
    }
    pub fn timestamp(&self) -> &str {
        &self.timestamp
    }
}