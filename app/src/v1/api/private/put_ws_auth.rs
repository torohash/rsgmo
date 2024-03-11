use crate::{
    v1::api::GmoApi,
    request::AccessLevel,
};

use anyhow::Result;
use serde::{Deserialize, Serialize};

const PATH: &str = "/v1/ws-auth";

impl GmoApi {
    pub async fn put_ws_auth(&self, parameters: PutWsAuthParameters) -> Result<PutWsAuthResponse> {
        self.put(PATH, Some(parameters), AccessLevel::Private).await
    }
}


#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PutWsAuthParameters {
    token: String,
}

impl PutWsAuthParameters {
    pub fn new(
        token: &str,
    ) -> Self {
        Self {
            token: token.to_string(),
        }
    }
}
#[derive(Debug, Clone, Deserialize)]
pub struct PutWsAuthResponse {
    status: i32,
    responsetime: String,
}

impl PutWsAuthResponse {
    pub fn status(&self) -> i32 {
        self.status
    }
    pub fn responsetime(&self) -> &str {
        &self.responsetime
    }
}