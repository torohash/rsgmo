use crate::{
    v1::api::GmoApi,
    request::AccessLevel,
    utils,
};

use anyhow::Result;
use serde::{Deserialize, Serialize};

const PATH: &str = "/v1/orders";

impl GmoApi {
    pub async fn get_orders(&self, parameters: GetOrdersParameters) -> Result<GetOrdersResponse> {
        self.get(PATH, Some(parameters), AccessLevel::Private).await
    }
}


#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetOrdersParameters {
    order_id: String,
}

impl GetOrdersParameters {
    pub fn new(order_id: &str) -> Self {
        GetOrdersParameters {
            order_id: order_id.to_string(),
        }
    }
}
#[derive(Debug, Clone, Deserialize)]
pub struct GetOrdersResponse {
    status: i32,
    data: OrdersData,
    responsetime: String,
}

impl GetOrdersResponse {
    pub fn status(&self) -> i32 {
        self.status
    }
    pub fn data(&self) -> &OrdersData {
        &self.data
    }
    pub fn responsetime(&self) -> &str {
        &self.responsetime
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrdersData {
    list: Option<Vec<Orders>>
}

impl OrdersData {
    pub fn list(&self) -> &Option<Vec<Orders>> {
        &self.list
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Orders {
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

impl Orders {
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