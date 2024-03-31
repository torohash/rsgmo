use crate::{
    v1::ws::{GmoWs, Channel, CommandType},
    connect::Connect,
    utils,
};

use anyhow::Result;
use serde::{Deserialize, Serialize};

impl GmoWs {
    pub async fn connect_order_events(&self, parameters: ConnectOrderEventsParameters) -> Result<()> {
        self.execute(parameters).await
    }
}


#[derive(Debug, Clone, Serialize)]
pub struct ConnectOrderEventsParameters {
    command: CommandType,
    channel: Channel,
}

impl ConnectOrderEventsParameters {
    pub fn new(command: CommandType) -> Self {
        Self {
            command,
            channel: Channel::OrderEvents,
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConnectOrderEventsResponse {
    channel: String,
    order_id: i64,
    symbol: String,
    settle_type: String,
    execution_type: String,
    side: String,
    order_status: String,
    cancel_type: Option<String>,
    order_timestamp: String,
    #[serde(deserialize_with = "utils::deserialize_f64")]
    order_price: f64,
    #[serde(deserialize_with = "utils::deserialize_f64")]
    order_size: f64,
    #[serde(deserialize_with = "utils::deserialize_f64")]
    order_executed_size: f64,
    #[serde(deserialize_with = "utils::deserialize_f64")]
    losscut_price: f64,
    time_in_force: String,
    msg_type: String,
}

impl ConnectOrderEventsResponse {
    pub fn channel(&self) -> &str {
        &self.channel
    }
    pub fn order_id(&self) -> i64 {
        self.order_id
    }
    pub fn symbol(&self) -> &str {
        &self.symbol
    }
    pub fn settle_type(&self) -> &str {
        &self.settle_type
    }
    pub fn execution_type(&self) -> &str {
        &self.execution_type
    }
    pub fn side(&self) -> &str {
        &self.side
    }
    pub fn order_status(&self) -> &str {
        &self.order_status
    }
    pub fn cancel_type(&self) -> &Option<String> {
        &self.cancel_type
    }
    pub fn order_timestamp(&self) -> &str {
        &self.order_timestamp
    }
    pub fn order_price(&self) -> f64 {
        self.order_price
    }
    pub fn order_size(&self) -> f64 {
        self.order_size
    }
    pub fn order_executed_size(&self) -> f64 {
        self.order_executed_size
    }
    pub fn losscut_price(&self) -> f64 {
        self.losscut_price
    }
    pub fn time_in_force(&self) -> &str {
        &self.time_in_force
    }
    pub fn msg_type(&self) -> &str {
        &self.msg_type
    }
}