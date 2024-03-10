use crate::{
    v1::api::GmoApi,
    request::AccessLevel,
    utils,
};

use anyhow::Result;
use serde::{Deserialize, Serialize};

const PATH: &str = "/v1/account/transfer";

impl GmoApi {
    pub async fn post_account_transfer(&self, parameters: PostAccountTransferParameters) -> Result<PostAccountTransferResponse> {
        self.post(PATH, Some(parameters), AccessLevel::Private).await
    }
}


#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PostAccountTransferParameters {
    #[serde(serialize_with = "utils::serialize_as_string")]
    amount: f64,
    transfer_type: String,
}

impl PostAccountTransferParameters {
    pub fn new(amount: f64, transfer_type: &str) -> Self {
        PostAccountTransferParameters {
            amount,
            transfer_type: transfer_type.to_string(),
        }
    }
}
#[derive(Debug, Clone, Deserialize)]
pub struct PostAccountTransferResponse {
    status: i32,
    data: AccountTransferData,
    responsetime: String,
}

impl PostAccountTransferResponse {
    pub fn status(&self) -> i32 {
        self.status
    }
    pub fn data(&self) -> &AccountTransferData {
        &self.data
    }
    pub fn responsetime(&self) -> &str {
        &self.responsetime
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountTransferData {
    #[serde(deserialize_with = "utils::deserialize_f64")]
    transferred_amount: f64,
}

impl AccountTransferData {
    pub fn transferred_amount(&self) -> f64 {
        self.transferred_amount
    }
}