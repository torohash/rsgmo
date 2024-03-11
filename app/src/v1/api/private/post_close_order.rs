use crate::{
    v1::api::GmoApi,
    request::AccessLevel,
    utils,
};

use anyhow::Result;
use serde::{Deserialize, Serialize};

const PATH: &str = "/v1/closeOrder";

impl GmoApi {
    pub async fn post_close_order(&self, parameters: PostCloseOrderParameters) -> Result<PostCloseOrderResponse> {
        self.post(PATH, Some(parameters), AccessLevel::Private).await
    }
}


#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PostCloseOrderParameters {
    symbol: String,
    side: String,
    execution_type: String,
    time_in_force: Option<String>,
    #[serde(serialize_with = "utils::serialize_option_as_string")]
    price: Option<f64>,
    settle_position: SettlePositionParameter,
    cancel_before: Option<bool>,
}

impl PostCloseOrderParameters {
    pub fn new(
        symbol: &str,
        side: &str,
        execution_type: &str,
        settle_position: SettlePositionParameter,
    ) -> Self {
        Self {
            symbol: symbol.to_string(),
            side: side.to_string(),
            execution_type: execution_type.to_string(),
            settle_position,
            price: None,
            time_in_force: None,
            cancel_before: None,
        }
    }

    pub fn with_time_in_force(mut self, time_in_force: &str) -> Self {
        self.time_in_force = Some(time_in_force.to_string());
        self
    }

    pub fn with_price(mut self, price: f64) -> Self {
        self.price = Some(price);
        self
    }

    pub fn with_cancel_before(mut self, cancel_before: bool) -> Self {
        self.cancel_before = Some(cancel_before);
        self
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SettlePositionParameter {
    position_id: i64,
    #[serde(serialize_with = "utils::serialize_as_string")]
    size: f64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PostCloseOrderResponse {
    status: i32,
    data: String,
    responsetime: String,
}

impl PostCloseOrderResponse {
    pub fn status(&self) -> i32 {
        self.status
    }
    pub fn data(&self) -> &str {
        &self.data
    }
    pub fn responsetime(&self) -> &str {
        &self.responsetime
    }
}