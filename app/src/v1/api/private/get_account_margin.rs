use crate::{
    v1::api::GmoApi,
    request::AccessLevel,
    utils,
};

use anyhow::Result;
use serde::Deserialize;

const PATH: &str = "/v1/account/assets";

impl GmoApi {
    pub async fn get_account_assets(&self) -> Result<GetAccountAssetsResponse> {
        self.get(PATH, None::<()>, AccessLevel::Private).await
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct GetAccountAssetsResponse {
    status: i32,
    data: Vec<AccountAssetsData>,
    responsetime: String,
}

impl GetAccountAssetsResponse {
    pub fn status(&self) -> i32 {
        self.status
    }
    pub fn data(&self) -> &Vec<AccountAssetsData> {
        &self.data
    }
    pub fn responsetime(&self) -> &str {
        &self.responsetime
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountAssetsData {
    #[serde(deserialize_with = "utils::deserialize_f64")]
    amount: f64,
    #[serde(deserialize_with = "utils::deserialize_f64")]
    available: f64,
    #[serde(deserialize_with = "utils::deserialize_f64")]
    conversion_rate: f64,
    symbol: String,
}

impl AccountAssetsData {
    pub fn amount(&self) -> f64 {
        self.amount
    }
    pub fn available(&self) -> f64 {
        self.available
    }
    pub fn conversion_rate(&self) -> f64 {
        self.conversion_rate
    }
    pub fn symbol(&self) -> &str {
        &self.symbol
    }
}