use crate::{
    v1::api::GmoApi,
    request::AccessLevel,
    utils,
};

use anyhow::Result;
use serde::Deserialize;

const PATH: &str = "/v1/account/margin";

impl GmoApi {
    pub async fn get_account_margin(&self) -> Result<GetAccountMarginResponse> {
        self.get(PATH, None::<()>, AccessLevel::Private).await
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct GetAccountMarginResponse {
    status: i32,
    data: AccountMarginData,
    responsetime: String,
}

impl GetAccountMarginResponse {
    pub fn status(&self) -> i32 {
        self.status
    }
    pub fn data(&self) -> &AccountMarginData {
        &self.data
    }
    pub fn responsetime(&self) -> &str {
        &self.responsetime
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountMarginData {
    #[serde(deserialize_with = "utils::deserialize_string_to_u64")]
    actual_profit_loss: u64,
    #[serde(deserialize_with = "utils::deserialize_string_to_u64")]
    available_amount: u64,
    #[serde(deserialize_with = "utils::deserialize_string_to_u64")]
    margin: u64,
    margin_call_status: String,
    #[serde(deserialize_with = "utils::deserialize_string_to_u64")]
    profit_loss: u64,
    #[serde(deserialize_with = "utils::deserialize_string_to_u64")]
    transferable_amount: u64,
}

impl AccountMarginData {
    pub fn actual_profit_loss(&self) -> u64 {
        self.actual_profit_loss
    }
    pub fn available_amount(&self) -> u64 {
        self.available_amount
    }
    pub fn margin(&self) -> u64 {
        self.margin
    }
    pub fn margin_call_status(&self) -> &str {
        &self.margin_call_status
    }
    pub fn profit_loss(&self) -> u64 {
        self.profit_loss
    }
    pub fn transferable_amount(&self) -> u64 {
        self.transferable_amount
    }
}