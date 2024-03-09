use crate::{
    v1::api::GmoApi,
    request::AccessLevel,
    utils,
};

use anyhow::Result;
use serde::{Deserialize, Serialize};

const PATH: &str = "/v1/orderbooks";

impl GmoApi {
    pub async fn get_orderbooks(&self, parameters: Option<GetOrderbooksParameters>) -> Result<GetOrderbooksResponse> {
        self.get(PATH, parameters, AccessLevel::Public).await
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct GetOrderbooksParameters {
    symbol: String,
}

impl GetOrderbooksParameters {
    pub fn new(symbol: &str) -> Self {
        GetOrderbooksParameters {
            symbol: symbol.to_string(),
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct GetOrderbooksResponse {
    status: i32,
    data: OrderbooksData,
    responsetime: String,
}

impl GetOrderbooksResponse {
    pub fn status(&self) -> i32 {
        self.status
    }
    pub fn data(&self) -> &OrderbooksData {
        &self.data
    }
    pub fn responsetime(&self) -> &str {
        &self.responsetime
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderbooksData {
    asks: Vec<Order>,
    bids: Vec<Order>,
    symbol: String,
}

impl OrderbooksData {
    pub fn asks(&self) -> &Vec<Order> {
        &self.asks
    }
    pub fn bids(&self) -> &Vec<Order> {
        &self.bids
    }
    pub fn symbol(&self) -> &str {
        &self.symbol
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Order {
    #[serde(deserialize_with = "utils::deserialize_string_to_u64")]
    price: u64,
    #[serde(deserialize_with = "utils::deserialize_f64")]
    size: f64,
}

impl Order {
    pub fn price(&self) -> u64 {
        self.price
    }
    pub fn size(&self) -> f64 {
        self.size
    }
}