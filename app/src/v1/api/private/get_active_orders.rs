use crate::{
    v1::api::{GmoApi, Pagination},
    request::AccessLevel,
    utils,
};

use anyhow::Result;
use serde::{Deserialize, Serialize};

const PATH: &str = "/v1/activeOrders";

impl GmoApi {
    pub async fn get_active_orders(&self, parameters: GetActiveOrdersParameters) -> Result<GetActiveOrdersResponse> {
        self.get(PATH, Some(parameters), AccessLevel::Private).await
    }
}


#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetActiveOrdersParameters {
    symbol: String,
    page: Option<i32>,
    count: Option<i32>,
}

impl GetActiveOrdersParameters {
    pub fn new(symbol: &str) -> Self {
        GetActiveOrdersParameters {
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
pub struct GetActiveOrdersResponse {
    status: i32,
    data: ActiveOrdersData,
    responsetime: String,
}

impl GetActiveOrdersResponse {
    pub fn status(&self) -> i32 {
        self.status
    }
    pub fn data(&self) -> &ActiveOrdersData {
        &self.data
    }
    pub fn responsetime(&self) -> &str {
        &self.responsetime
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActiveOrdersData {
    pagination: Option<Pagination>,
    list: Option<Vec<ActiveOrders>>,
}

impl ActiveOrdersData {
    pub fn pagination(&self) -> &Option<Pagination> {
        &self.pagination
    }
    pub fn list(&self) -> &Option<Vec<ActiveOrders>> {
        &self.list
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActiveOrders {
    order_id: i64,
    root_order_id: i64,
    symbol: String,
    side: String,
    order_type: String,
    settle_type: String,
    #[serde(deserialize_with = "utils::deserialize_f64")]
    size: f64,
    #[serde(deserialize_with = "utils::deserialize_f64")]
    executed_size: f64,
    #[serde(deserialize_with = "utils::deserialize_f64")]
    pride: f64,
    #[serde(deserialize_with = "utils::deserialize_f64")]
    losscut_price: f64,
    status: String,
    time_in_force: String,
    timestamp: String,
}

impl ActiveOrders {
    pub fn order_id(&self) -> i64 {
        self.order_id
    }
    pub fn root_order_id(&self) -> i64 {
        self.root_order_id
    }
    pub fn symbol(&self) -> &str {
        &self.symbol
    }
    pub fn side(&self) -> &str {
        &self.side
    }
    pub fn order_type(&self) -> &str {
        &self.order_type
    }
    pub fn settle_type(&self) -> &str {
        &self.settle_type
    }
    pub fn size(&self) -> f64 {
        self.size
    }
    pub fn executed_size(&self) -> f64 {
        self.executed_size
    }
    pub fn pride(&self) -> f64 {
        self.pride
    }
    pub fn losscut_price(&self) -> f64 {
        self.losscut_price
    }
    pub fn status(&self) -> &str {
        &self.status
    }
    pub fn time_in_force(&self) -> &str {
        &self.time_in_force
    }
    pub fn timestamp(&self) -> &str {
        &self.timestamp
    }
}