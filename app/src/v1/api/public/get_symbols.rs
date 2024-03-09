use crate::{
    v1::api::GmoApi,
    request::AccessLevel,
    utils,
};

use anyhow::Result;
use serde::Deserialize;

const PATH: &str = "/v1/symbols";

impl GmoApi {
    pub async fn get_symbols(&self) -> Result<GetSymbolsResponse> {
        self.get(PATH, None::<()>, AccessLevel::Public).await
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct GetSymbolsResponse {
    status: i32,
    data: Vec<SymbolsData>,
    responsetime: String,
}

impl GetSymbolsResponse {
    pub fn status(&self) -> i32 {
        self.status
    }
    pub fn data(&self) -> &Vec<SymbolsData> {
        &self.data
    }
    pub fn responsetime(&self) -> &str {
        &self.responsetime
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SymbolsData {
    symbol: String,
    #[serde(deserialize_with = "utils::deserialize_f64")]
    min_order_size: f64,
    #[serde(deserialize_with = "utils::deserialize_f64")]
    max_order_size: f64,
    #[serde(deserialize_with = "utils::deserialize_f64")]
    size_step: f64,
    #[serde(deserialize_with = "utils::deserialize_f64")]
    tick_size: f64,
    #[serde(deserialize_with = "utils::deserialize_f64")]
    taker_fee: f64,
    #[serde(deserialize_with = "utils::deserialize_f64")]
    maker_fee: f64,
}

impl SymbolsData {
    pub fn symbol(&self) -> &str {
        &self.symbol
    }
    pub fn min_order_size(&self) -> f64 {
        self.min_order_size
    }
    pub fn max_order_size(&self) -> f64 {
        self.max_order_size
    }
    pub fn size_step(&self) -> f64 {
        self.size_step
    }
    pub fn tick_size(&self) -> f64 {
        self.tick_size
    }
    pub fn taker_fee(&self) -> f64 {
        self.taker_fee
    }
    pub fn maker_fee(&self) -> f64 {
        self.maker_fee
    }
}