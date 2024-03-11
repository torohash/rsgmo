use crate::{
    v1::api::GmoApi,
    request::AccessLevel,
    utils,
};

use anyhow::Result;
use serde::{Deserialize, Serialize};

const PATH: &str = "/v1/changeOrder";

impl GmoApi {
    pub async fn post_change_order(&self, parameters: PostChangeOrderParameters) -> Result<PostChangeOrderResponse> {
        self.post(PATH, Some(parameters), AccessLevel::Private).await
    }
}


#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PostChangeOrderParameters {
    order_id: String,
    #[serde(serialize_with = "utils::serialize_as_string")]
    price: f64,
    #[serde(serialize_with = "utils::serialize_option_as_string")]
    losscut_price: Option<f64>
}

impl PostChangeOrderParameters {
    pub fn new(
        order_id: &str,
        price: f64,
    ) -> Self {
        Self {
            order_id: order_id.to_string(),
            price,
            losscut_price: None,
        }
    }

    pub fn with_losscut_price(mut self, losscut_price: f64) -> Self {
        self.losscut_price = Some(losscut_price);
        self
    }
}
#[derive(Debug, Clone, Deserialize)]
pub struct PostChangeOrderResponse {
    status: i32,
    responsetime: String,
}

impl PostChangeOrderResponse {
    pub fn status(&self) -> i32 {
        self.status
    }
    pub fn responsetime(&self) -> &str {
        &self.responsetime
    }
}