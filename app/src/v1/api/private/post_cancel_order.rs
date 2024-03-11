use crate::{
    v1::api::GmoApi,
    request::AccessLevel,
};

use anyhow::Result;
use serde::{Deserialize, Serialize};

const PATH: &str = "/v1/cancelOrder";

impl GmoApi {
    pub async fn post_cancel_order(&self, parameters: PostCancelOrderParameters) -> Result<PostCancelOrderResponse> {
        self.post(PATH, Some(parameters), AccessLevel::Private).await
    }
}


#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PostCancelOrderParameters {
    order_id: String,
}

impl PostCancelOrderParameters {
    pub fn new(
        order_id: &str,
    ) -> Self {
        Self {
            order_id: order_id.to_string(),
        }
    }
}
#[derive(Debug, Clone, Deserialize)]
pub struct PostCancelOrderResponse {
    status: i32,
    responsetime: String,
}

impl PostCancelOrderResponse {
    pub fn status(&self) -> i32 {
        self.status
    }
    pub fn responsetime(&self) -> &str {
        &self.responsetime
    }
}