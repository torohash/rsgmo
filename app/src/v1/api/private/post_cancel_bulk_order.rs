use crate::{
    v1::api::GmoApi,
    request::AccessLevel,
};

use anyhow::Result;
use serde::{Deserialize, Serialize};

const PATH: &str = "/v1/cancelBulkOrder";

impl GmoApi {
    pub async fn post_cancel_bulk_order(&self, parameters: PostCancelBulkOrderParameters) -> Result<PostCancelBulkOrderResponse> {
        self.post(PATH, Some(parameters), AccessLevel::Private).await
    }
}


#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PostCancelBulkOrderParameters {
    symbols: String,
    side: Option<String>,
    settle_type: Option<String>,
    desc: Option<bool>
}

impl PostCancelBulkOrderParameters {
    pub fn new(
        symbols: &str,
    ) -> Self {
        Self {
            symbols: symbols.to_string(),
            side: None,
            settle_type: None,
            desc: None
        }
    }

    pub fn with_side(mut self, side: &str) -> Self {
        self.side = Some(side.to_string());
        self
    }

    pub fn with_settle_type(mut self, settle_type: &str) -> Self {
        self.settle_type = Some(settle_type.to_string());
        self
    }

    pub fn with_desc(mut self, desc: bool) -> Self {
        self.desc = Some(desc);
        self
    }
}
#[derive(Debug, Clone, Deserialize)]
pub struct PostCancelBulkOrderResponse {
    status: i32,
    data: Vec<i64>,
    responsetime: String,
}

impl PostCancelBulkOrderResponse {
    pub fn status(&self) -> i32 {
        self.status
    }
    pub fn data(&self) -> &Vec<i64> {
        &self.data
    }
    pub fn responsetime(&self) -> &str {
        &self.responsetime
    }
}