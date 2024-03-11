use crate::{
    v1::api::GmoApi,
    request::AccessLevel,
    utils,
};

use anyhow::Result;
use serde::{Deserialize, Serialize};

const PATH: &str = "/v1/order";

impl GmoApi {
    pub async fn post_order(&self, parameters: PostOrderParameters) -> Result<PostOrderResponse> {
        self.post(PATH, Some(parameters), AccessLevel::Private).await
    }
}


#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PostOrderParameters {
    symbol: String,
    side: String,
    execution_type: String,
    time_in_force: Option<String>,
    #[serde(serialize_with = "utils::serialize_option_as_string")]
    price: Option<f64>,
    #[serde(serialize_with = "utils::serialize_option_as_string")]
    losscut_price: Option<f64>,
    #[serde(serialize_with = "utils::serialize_as_string")]
    size: f64,
    cancel_before: Option<bool>,
}

impl PostOrderParameters {
    pub fn new(
        symbol: &str,
        side: &str,
        execution_type: &str,
        size: f64,
    ) -> Self {
        Self {
            symbol: symbol.to_string(),
            side: side.to_string(),
            execution_type: execution_type.to_string(),
            time_in_force: None,
            price: None,
            losscut_price: None,
            size: size,
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

    pub fn with_losscut_price(mut self, losscut_price: f64) -> Self {
        self.losscut_price = Some(losscut_price);
        self
    }

    pub fn with_cancel_before(mut self, cancel_before: bool) -> Self {
        self.cancel_before = Some(cancel_before);
        self
    }
}
#[derive(Debug, Clone, Deserialize)]
pub struct PostOrderResponse {
    status: i32,
    data: String,
    responsetime: String,
}

impl PostOrderResponse {
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