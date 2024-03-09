use crate::{
    v1::api::GmoApi,
    request::AccessLevel,
};

use anyhow::Result;
use serde::Deserialize;

const PATH: &str = "/v1/status";

impl GmoApi {
    pub async fn get_exchange_status(&self) -> Result<GetExchangeStatusResponse> {
        self.get(PATH, None::<()>, AccessLevel::Public).await
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct GetExchangeStatusResponse {
    status: i32,
    data: ExchangeStatusData,
    responsetime: String,
}

impl GetExchangeStatusResponse {
    pub fn status(&self) -> i32 {
        self.status
    }
    pub fn data(&self) -> &ExchangeStatusData {
        &self.data
    }
    pub fn responsetime(&self) -> &str {
        &self.responsetime
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExchangeStatusData {
    status: String,
}

impl ExchangeStatusData {
    pub fn status(&self) -> &str {
        &self.status
    }
}