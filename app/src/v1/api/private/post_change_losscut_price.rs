use crate::{
    v1::api::GmoApi,
    request::AccessLevel,
    utils,
};

use anyhow::Result;
use serde::{Deserialize, Serialize};

const PATH: &str = "/v1/changeLosscutPrice";

impl GmoApi {
    pub async fn post_change_losscut_price(&self, parameters: PostChangeLosscutPriceParameters) -> Result<PostChangeLosscutPriceResponse> {
        self.post(PATH, Some(parameters), AccessLevel::Private).await
    }
}


#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PostChangeLosscutPriceParameters {
    position_id: i64,
    #[serde(serialize_with = "utils::serialize_as_string")]
    losscut_price: f64,
}

impl PostChangeLosscutPriceParameters {
    pub fn new(
        position_id: i64,
        losscut_price: f64,
    ) -> Self {
        Self {
            position_id,
            losscut_price,
        }
    }

}
#[derive(Debug, Clone, Deserialize)]
pub struct PostChangeLosscutPriceResponse {
    status: i32,
    responsetime: String,
}

impl PostChangeLosscutPriceResponse {
    pub fn status(&self) -> i32 {
        self.status
    }
    pub fn responsetime(&self) -> &str {
        &self.responsetime
    }
}