use crate::{
    v1::api::GmoApi,
    request::AccessLevel,
    utils,
};

use anyhow::Result;
use serde::{Deserialize, Serialize};

const PATH: &str = "/v1/positionSummary";

impl GmoApi {
    pub async fn get_position_summary(&self, parameters: GetPositionSummaryParameters) -> Result<GetPositionSummaryResponse> {
        self.get(PATH, Some(parameters), AccessLevel::Private).await
    }
}


#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetPositionSummaryParameters {
    symbol: Option<String>,
}

impl GetPositionSummaryParameters {
    pub fn new() -> Self {
        GetPositionSummaryParameters {
            symbol: None,
        }
    }

    pub fn with_symbol(mut self, symbol: &str) -> Self {
        self.symbol = Some(symbol.to_string());
        self
    }
}
#[derive(Debug, Clone, Deserialize)]
pub struct GetPositionSummaryResponse {
    status: i32,
    data: PositionSummaryData,
    responsetime: String,
}

impl GetPositionSummaryResponse {
    pub fn status(&self) -> i32 {
        self.status
    }
    pub fn data(&self) -> &PositionSummaryData {
        &self.data
    }
    pub fn responsetime(&self) -> &str {
        &self.responsetime
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PositionSummaryData {
    list: Option<Vec<PositionSummary>>
}

impl PositionSummaryData {
    pub fn list(&self) -> &Option<Vec<PositionSummary>> {
        &self.list
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PositionSummary {
    #[serde(deserialize_with = "utils::deserialize_f64")]
    average_position_rate: f64,
    #[serde(deserialize_with = "utils::deserialize_f64")]
    position_loss_gain: f64,
    side: String,
    #[serde(deserialize_with = "utils::deserialize_f64")]
    sum_order_quantity: f64,
    #[serde(deserialize_with = "utils::deserialize_f64")]
    sum_position_quantity: f64,
    symbol: String,
}

impl PositionSummary {
    pub fn average_position_rate(&self) -> f64 {
        self.average_position_rate
    }
    pub fn position_loss_gain(&self) -> f64 {
        self.position_loss_gain
    }
    pub fn side(&self) -> &str {
        &self.side
    }
    pub fn sum_order_quantity(&self) -> f64 {
        self.sum_order_quantity
    }
    pub fn sum_position_quantity(&self) -> f64 {
        self.sum_position_quantity
    }
    pub fn symbol(&self) -> &str {
        &self.symbol
    }
}