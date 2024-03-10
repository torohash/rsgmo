use crate::{
    v1::api::GmoApi,
    request::AccessLevel,
    utils,
};

use anyhow::Result;
use serde::{Serialize, Deserialize};

const PATH: &str = "/v1/account/withdrawal/history";

impl GmoApi {
    pub async fn get_account_withdrawal_history(&self, parameters: GetWithdrawalHistoryParameters) -> Result<GetAccountWithdrawalHistoryResponse> {
        self.get(PATH, Some(parameters), AccessLevel::Private).await
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetWithdrawalHistoryParameters {
    symbol: String,
    from_timestamp: String,
    to_timestamp: Option<String>,
}

impl GetWithdrawalHistoryParameters {
    pub fn new(symbol: &str, from_timestamp: &str) -> Self {
        GetWithdrawalHistoryParameters {
            symbol: symbol.to_string(),
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
pub struct GetAccountWithdrawalHistoryResponse {
    status: i32,
    data: Vec<AccountWithdrawalHistoryData>,
    responsetime: String,
}

impl GetAccountWithdrawalHistoryResponse {
    pub fn status(&self) -> i32 {
        self.status
    }
    pub fn data(&self) -> &Vec<AccountWithdrawalHistoryData> {
        &self.data
    }
    pub fn responsetime(&self) -> &str {
        &self.responsetime
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountWithdrawalHistoryData {
    address: String,
    #[serde(deserialize_with = "utils::deserialize_f64")]
    amount: f64,
    #[serde(deserialize_with = "utils::deserialize_option_f64")]
    fee: Option<f64>,
    status: String,
    symbol: String,
    timestamp: String,
    tx_hash: String,
}

impl AccountWithdrawalHistoryData {
    pub fn address(&self) -> &str {
        &self.address
    }
    pub fn amount(&self) -> f64 {
        self.amount
    }
    pub fn fee(&self) -> Option<f64> {
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
    pub fn tx_hash(&self) -> &str {
        &self.tx_hash
    }
}