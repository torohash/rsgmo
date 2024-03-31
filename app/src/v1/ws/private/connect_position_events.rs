use crate::{
    v1::ws::{GmoWs, Channel, CommandType},
    connect::Connect,
    utils,
};

use anyhow::Result;
use serde::{Deserialize, Serialize};

impl GmoWs {
    pub async fn connect_position_events(&self, parameters: ConnectPositionEventsParameters) -> Result<()> {
        self.execute(parameters).await
    }
}


#[derive(Debug, Clone, Serialize)]
pub struct ConnectPositionEventsParameters {
    command: CommandType,
    channel: Channel,
}

impl ConnectPositionEventsParameters {
    pub fn new(command: CommandType) -> Self {
        Self {
            command,
            channel: Channel::PositionEvents,
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConnectPositionEventsResponse {
    channel: String,
    position_id: i64,
    symbol: String,
    side: String,
    #[serde(deserialize_with = "utils::deserialize_f64")]
    size: f64,
    #[serde(deserialize_with = "utils::deserialize_f64")]
    order_size: f64,
    #[serde(deserialize_with = "utils::deserialize_f64")]
    price: f64,
    #[serde(deserialize_with = "utils::deserialize_f64")]
    loss_gain: f64,
    leverage: String,
    #[serde(deserialize_with = "utils::deserialize_f64")]
    losscut_price: f64,
    timestamp: String,
    msg_type: String,
}

impl ConnectPositionEventsResponse {
    pub fn channel(&self) -> &str {
        &self.channel
    }
    pub fn position_id(&self) -> i64 {
        self.position_id
    }
    pub fn symbol(&self) -> &str {
        &self.symbol
    }
    pub fn side(&self) -> &str {
        &self.side
    }
    pub fn size(&self) -> f64 {
        self.size
    }
    pub fn order_size(&self) -> f64 {
        self.order_size
    }
    pub fn price(&self) -> f64 {
        self.price
    }
    pub fn loss_gain(&self) -> f64 {
        self.loss_gain
    }
    pub fn leverage(&self) -> &str {
        &self.leverage
    }
    pub fn losscut_price(&self) -> f64 {
        self.losscut_price
    }
    pub fn timestamp(&self) -> &str {
        &self.timestamp
    }
    pub fn msg_type(&self) -> &str {
        &self.msg_type
    }
}