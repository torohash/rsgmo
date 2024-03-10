use crate::{
    v1::api::GmoApi,
    request::AccessLevel,
    utils,
};

use anyhow::Result;
use serde::{Deserialize, Serialize};

const PATH: &str = "/v1/executions";

impl GmoApi {
    pub async fn get_executions(&self, parameters: GetExecutionsParameters) -> Result<GetExecutionsResponse> {
        self.get(PATH, Some(parameters), AccessLevel::Private).await
    }
}


#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetExecutionsParameters {
    order_id: Option<String>,
    execution_id: Option<String>,
}

impl GetExecutionsParameters {
    pub fn new() -> Self {
        GetExecutionsParameters {
            order_id: None,
            execution_id: None,
        }
    }

    pub fn with_order_id(mut self, order_id: &str) -> Self {
        self.order_id = Some(order_id.to_string());
        self
    }

    pub fn with_execution_id(mut self, execution_id: &str) -> Self {
        self.execution_id = Some(execution_id.to_string());
        self
    }
}
#[derive(Debug, Clone, Deserialize)]
pub struct GetExecutionsResponse {
    status: i32,
    data: ExecutionsData,
    responsetime: String,
}

impl GetExecutionsResponse {
    pub fn status(&self) -> i32 {
        self.status
    }
    pub fn data(&self) -> &ExecutionsData {
        &self.data
    }
    pub fn responsetime(&self) -> &str {
        &self.responsetime
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExecutionsData {
    list: Option<Vec<Executions>>
}

impl ExecutionsData {
    pub fn list(&self) -> &Option<Vec<Executions>> {
        &self.list
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Executions {
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

impl Executions {
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