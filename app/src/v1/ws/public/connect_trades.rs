use crate::{
    v1::ws::{GmoWs, Channel, CommandType},
    connect::Connect,
    utils,
};

use anyhow::Result;
use serde::{Deserialize, Serialize};

impl GmoWs {
    pub async fn connect_trades(&self, parameters: ConnectTradesParameters) -> Result<()> {
        self.execute(parameters).await
    }
}


#[derive(Debug, Clone, Serialize)]
pub struct ConnectTradesParameters {
    command: CommandType,
    channel: Channel,
    symbol: String,
    option: Option<String>,
}


impl ConnectTradesParameters {
    pub fn new(command: CommandType, symbol: &str) -> Self {
        Self {
            command,
            channel: Channel::Trades,
            symbol: symbol.to_string(),
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
pub struct ConnectTradesResponse {
    channel: String,
    #[serde(deserialize_with = "utils::deserialize_f64")]
    price: f64,
    side: String,
    #[serde(deserialize_with = "utils::deserialize_f64")]
    size: f64,
    timestamp: String,
    symbol: String,
}

impl ConnectTradesResponse {
    pub fn channel(&self) -> &str {
        &self.channel
    }
    pub fn price(&self) -> f64 {
        self.price
    }
    pub fn side(&self) -> &str {
        &self.side
    }
    pub fn size(&self) -> f64 {
        self.size
    }
    pub fn timestamp(&self) -> &str {
        &self.timestamp
    }
    pub fn symbol(&self) -> &str {
        &self.symbol
    }
}