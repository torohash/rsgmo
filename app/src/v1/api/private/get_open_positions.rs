use crate::{
    v1::api::{GmoApi, Pagination},
    request::AccessLevel,
    utils,
};

use anyhow::Result;
use serde::{Deserialize, Serialize};

const PATH: &str = "/v1/openPositions";

impl GmoApi {
    pub async fn get_open_positions(&self, parameters: GetOpenPositionsParameters) -> Result<GetOpenPositionsResponse> {
        self.get(PATH, Some(parameters), AccessLevel::Private).await
    }
}


#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetOpenPositionsParameters {
    symbol: String,
    page: Option<i32>,
    count: Option<i32>,
}

impl GetOpenPositionsParameters {
    pub fn new(symbol: &str) -> Self {
        GetOpenPositionsParameters {
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
pub struct GetOpenPositionsResponse {
    status: i32,
    data: OpenPositionsData,
    responsetime: String,
}

impl GetOpenPositionsResponse {
    pub fn status(&self) -> i32 {
        self.status
    }
    pub fn data(&self) -> &OpenPositionsData {
        &self.data
    }
    pub fn responsetime(&self) -> &str {
        &self.responsetime
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OpenPositionsData {
    pagination: Option<Pagination>,
    list: Option<Vec<OpenPositions>>
}

impl OpenPositionsData {
    pub fn pagination(&self) -> &Option<Pagination> {
        &self.pagination
    }
    pub fn list(&self) -> &Option<Vec<OpenPositions>> {
        &self.list
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OpenPositions {
    position_id: i64,
    symbol: String,
    side: String,
    #[serde(deserialize_with = "utils::deserialize_f64")]
    size: f64,
    #[serde(deserialize_with = "utils::deserialize_f64")]
    orderd_size: f64,
    #[serde(deserialize_with = "utils::deserialize_f64")]
    price: f64,
    #[serde(deserialize_with = "utils::deserialize_f64")]
    loss_gain: f64,
    #[serde(deserialize_with = "utils::deserialize_f64")]
    leverage: f64,
    #[serde(deserialize_with = "utils::deserialize_f64")]
    losscut_price: f64,
    timestamp: String,
}

impl OpenPositions {
    pub fn position_id(&self) -> i64 {
        self.position_id
    }
    pub fn symbol(&self) -> &str {
        &self.symbol
    }
    pub fn side(&self) -> &str {
        &self.side
    }
    pub fn size(&self) -> f64 {
        self.size
    }
    pub fn orderd_size(&self) -> f64 {
        self.orderd_size
    }
    pub fn price(&self) -> f64 {
        self.price
    }
    pub fn loss_gain(&self) -> f64 {
        self.loss_gain
    }
    pub fn leverage(&self) -> f64 {
        self.leverage
    }
    pub fn losscut_price(&self) -> f64 {
        self.losscut_price
    }
    pub fn timestamp(&self) -> &str {
        &self.timestamp
    }
}