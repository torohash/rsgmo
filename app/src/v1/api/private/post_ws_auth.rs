use crate::{
    v1::api::GmoApi,
    request::AccessLevel,
};

use anyhow::Result;
use serde::Deserialize;

const PATH: &str = "/v1/ws-auth";

impl GmoApi {
    pub async fn post_ws_auth(&self) -> Result<PostWsAuthResponse> {
        self.post(PATH, None::<()>, AccessLevel::Private).await
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct PostWsAuthResponse {
    status: i32,
    data: String,
    responsetime: String,
}

impl PostWsAuthResponse {
    pub fn status(&self) -> i32 {
        self.status
    }
    pub fn data(&self) -> &str {
        &self.data
    }
    pub fn responsetime(&self) -> &str {
        &self.responsetime
    }
}