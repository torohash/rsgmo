use crate::{
    v1::api::{GmoApi, Pagination},
    request::AccessLevel,
    utils,
};

use anyhow::Result;
use serde::{Deserialize, Serialize};

const PATH: &str = "/v1/latestExecutions";

impl GmoApi {
    pub async fn get_latest_executions(&self, parameters: GetLatestExecutionsParameters) -> Result<GetLatestExecutionsResponse> {
        self.get(PATH, Some(parameters), AccessLevel::Private).await
    }
}


#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetLatestExecutionsParameters {
    symbol: String,
    page: Option<i32>,
    count: Option<i32>,
}

impl GetLatestExecutionsParameters {
    pub fn new(symbol: &str) -> Self {
        GetLatestExecutionsParameters {
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
pub struct GetLatestExecutionsResponse {
    status: i32,
    data: LatestExecutionsData,
    responsetime: String,
}

impl GetLatestExecutionsResponse {
    pub fn status(&self) -> i32 {
        self.status
    }
    pub fn data(&self) -> &LatestExecutionsData {
        &self.data
    }
    pub fn responsetime(&self) -> &str {
        &self.responsetime
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LatestExecutionsData {
    pagination: Option<Pagination>,
    list: Option<Vec<LatestExecutions>>
}

impl LatestExecutionsData {
    pub fn pagination(&self) -> &Option<Pagination> {
        &self.pagination
    }
    pub fn list(&self) -> &Option<Vec<LatestExecutions>> {
        &self.list
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LatestExecutions {
    execution_id: i64,
    order_id: i64,
    position_id: i64,
    symbol: String,
    side: String,
    settle_type: String,
    #[serde(deserialize_with = "utils::deserialize_f64")]
    size: f64,
    #[serde(deserialize_with = "utils::deserialize_f64")]
    price: f64,
    #[serde(deserialize_with = "utils::deserialize_f64")]
    loss_gain: f64,
    #[serde(deserialize_with = "utils::deserialize_f64")]
    fee: f64,
    timestamp: String,
}

impl LatestExecutions {
    pub fn execution_id(&self) -> i64 {
        self.execution_id
    }
    pub fn order_id(&self) -> i64 {
        self.order_id
    }
    pub fn position_id(&self) -> i64 {
        self.position_id
    }
    pub fn symbol(&self) -> &str {
        &self.symbol
    }
    pub fn side(&self) -> &str {
        &self.side
    }
    pub fn settle_type(&self) -> &str {
        &self.settle_type
    }
    pub fn size(&self) -> f64 {
        self.size
    }
    pub fn price(&self) -> f64 {
        self.price
    }
    pub fn loss_gain(&self) -> f64 {
        self.loss_gain
    }
    pub fn fee(&self) -> f64 {
        self.fee
    }
    pub fn timestamp(&self) -> &str {
        &self.timestamp
    }
}