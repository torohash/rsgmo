use crate::{
    v1::api::GmoApi,
    request::AccessLevel,
    utils,
};
use serde_json::Value;

use anyhow::Result;
use serde::Deserialize;

const PATH: &str = "/v1/account/tradingVolume";

impl GmoApi {
    pub async fn get_account_trading_volume(&self) -> Result<GetAccountTradingVolumeResponse> {
        self.get(PATH, None::<()>, AccessLevel::Private).await
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct GetAccountTradingVolumeResponse {
    status: i32,
    data: AccountTradingVolumeData,
    responsetime: String,
}

impl GetAccountTradingVolumeResponse {
    pub fn status(&self) -> i32 {
        self.status
    }
    pub fn data(&self) -> &AccountTradingVolumeData {
        &self.data
    }
    pub fn responsetime(&self) -> &str {
        &self.responsetime
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountTradingVolumeData {
    #[serde(deserialize_with = "utils::deserialize_string_to_u64")]
    jpy_volume: u64,
    tier_level: i32,
    limit: Vec<Value>
}

impl AccountTradingVolumeData {
    pub fn jpy_volume(&self) -> u64 {
        self.jpy_volume
    }
    pub fn tier_level(&self) -> i32 {
        self.tier_level
    }
    pub fn limit(&self) -> &Vec<Value> {
        &self.limit
    }
}