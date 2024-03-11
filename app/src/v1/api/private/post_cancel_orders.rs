use crate::{
    v1::api::GmoApi,
    request::AccessLevel,
};

use anyhow::Result;
use serde::{Deserialize, Serialize};

const PATH: &str = "/v1/cancelOrders";

impl GmoApi {
    pub async fn post_cancel_orders(&self, parameters: PostCancelOrdersParameters) -> Result<PostCancelOrdersResponse> {
        self.post(PATH, Some(parameters), AccessLevel::Private).await
    }
}


#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PostCancelOrdersParameters {
    order_ids: String,
}

impl PostCancelOrdersParameters {
    pub fn new(
        order_ids: &str,
    ) -> Self {
        Self {
            order_ids: order_ids.to_string(),
        }
    }
}
#[derive(Debug, Clone, Deserialize)]
pub struct PostCancelOrdersResponse {
    status: i32,
    data: CancelOrdersData,
    responsetime: String,
}

impl PostCancelOrdersResponse {
    pub fn status(&self) -> i32 {
        self.status
    }
    pub fn data(&self) -> &CancelOrdersData {
        &self.data
    }
    pub fn responsetime(&self) -> &str {
        &self.responsetime
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelOrdersData {
    success: Vec<i64>,
    failed: Vec<FailOrderDetail>,
}

impl CancelOrdersData {
    pub fn success(&self) -> &Vec<i64> {
        &self.success
    }
    pub fn failed(&self) -> &Vec<FailOrderDetail> {
        &self.failed
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FailOrderDetail {
    message_code: String,
    message_string: String,
    order_id: i64,
}

impl FailOrderDetail {
    pub fn message_code(&self) -> &str {
        &self.message_code
    }
    pub fn message_string(&self) -> &str {
        &self.message_string
    }
    pub fn order_id(&self) -> i64 {
        self.order_id
    }
}