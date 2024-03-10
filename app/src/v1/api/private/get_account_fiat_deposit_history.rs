use crate::{
    v1::api::GmoApi,
    request::AccessLevel,
    utils,
};

use anyhow::Result;
use serde::{Serialize, Deserialize};

const PATH: &str = "/v1/account/fiatDeposit/history";

impl GmoApi {
    pub async fn get_account_fiat_deposit_history(&self, parameters: GetFiatDepositHistoryParameters) -> Result<GetAccountFiatDepositHistoryResponse> {
        self.get(PATH, Some(parameters), AccessLevel::Private).await
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetFiatDepositHistoryParameters {
    from_timestamp: String,
    to_timestamp: Option<String>,
}

impl GetFiatDepositHistoryParameters {
    pub fn new(from_timestamp: &str) -> Self {
        GetFiatDepositHistoryParameters {
            from_timestamp: from_timestamp.to_string(),
            to_timestamp: None,
        }
    }

    pub fn with_to_timestamp(mut self, to_timestamp: &str) -> Self {
        self.to_timestamp = Some(to_timestamp.to_string());
        self
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct GetAccountFiatDepositHistoryResponse {
    status: i32,
    data: Vec<AccountFiatDepositHistoryData>,
    responsetime: String,
}

impl GetAccountFiatDepositHistoryResponse {
    pub fn status(&self) -> i32 {
        self.status
    }
    pub fn data(&self) -> &Vec<AccountFiatDepositHistoryData> {
        &self.data
    }
    pub fn responsetime(&self) -> &str {
        &self.responsetime
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountFiatDepositHistoryData {
    #[serde(deserialize_with = "utils::deserialize_f64")]
    amount: f64,
    #[serde(deserialize_with = "utils::deserialize_f64")]
    fee: f64,
    status: String,
    symbol: String,
    timestamp: String
}

impl AccountFiatDepositHistoryData {
    pub fn amount(&self) -> f64 {
        self.amount
    }
    pub fn fee(&self) -> f64 {
        self.fee
    }
    pub fn status(&self) -> &str {
        &self.status
    }
    pub fn symbol(&self) -> &str {
        &self.symbol
    }
    pub fn timestamp(&self) -> &str {
        &self.timestamp
    }
}