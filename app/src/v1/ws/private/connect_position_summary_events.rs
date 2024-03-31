use crate::{
    v1::ws::{GmoWs, Channel, CommandType},
    connect::Connect,
};

use anyhow::Result;
use serde::{Deserialize, Serialize};

impl GmoWs {
    pub async fn connect_position_summary_events(&self, parameters: ConnectPositionSummaryEventsParameters) -> Result<()> {
        self.execute(parameters).await
    }
}


#[derive(Debug, Clone, Serialize)]
pub struct ConnectPositionSummaryEventsParameters {
    command: CommandType,
    channel: Channel,
    option: Option<String>,
}

impl ConnectPositionSummaryEventsParameters {
    pub fn new(command: CommandType) -> Self {
        Self {
            command,
            channel: Channel::PositionSummaryEvents,
            option: None,
        }
    }

    pub fn with_option(mut self, option: &str) -> Self {
        self.option = Some(option.to_string());
        self
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConnectPositionSummaryEventsResponse {
    channel: String,
    symbol: String,
    side: String,
    average_position_rate: String,
    position_loss_gain: String,
    sum_order_quantity: String,
    sum_position_quantity: String,
    timestamp: String,
    msg_type: String,
}

impl ConnectPositionSummaryEventsResponse {
    pub fn channel(&self) -> &str {
        &self.channel
    }
    pub fn symbol(&self) -> &str {
        &self.symbol
    }
    pub fn side(&self) -> &str {
        &self.side
    }
    pub fn average_position_rate(&self) -> &str {
        &self.average_position_rate
    }
    pub fn position_loss_gain(&self) -> &str {
        &self.position_loss_gain
    }
    pub fn sum_order_quantity(&self) -> &str {
        &self.sum_order_quantity
    }
    pub fn sum_position_quantity(&self) -> &str {
        &self.sum_position_quantity
    }
    pub fn timestamp(&self) -> &str {
        &self.timestamp
    }
    pub fn msg_type(&self) -> &str {
        &self.msg_type
    }
}