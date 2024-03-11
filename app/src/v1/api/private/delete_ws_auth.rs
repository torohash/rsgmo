use crate::{
    v1::api::GmoApi,
    request::AccessLevel,
};

use anyhow::Result;
use serde::{Deserialize, Serialize};

const PATH: &str = "/v1/ws-auth";

impl GmoApi {
    pub async fn delete_ws_auth(&self, parameters: DeleteWsAuthParameters) -> Result<DeleteWsAuthResponse> {
        self.delete(PATH, Some(parameters), AccessLevel::Private).await
    }
}


#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteWsAuthParameters {
    token: String,
}

impl DeleteWsAuthParameters {
    pub fn new(
        token: &str,
    ) -> Self {
        Self {
            token: token.to_string(),
        }
    }
}
#[derive(Debug, Clone, Deserialize)]
pub struct DeleteWsAuthResponse {
    status: i32,
    responsetime: String,
}

impl DeleteWsAuthResponse {
    pub fn status(&self) -> i32 {
        self.status
    }
    pub fn responsetime(&self) -> &str {
        &self.responsetime
    }
}