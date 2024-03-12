use crate::{
    v1::ws::{GmoWs, Channel, CommandType},
    connect::Connect,
    utils,
};
use tokio_tungstenite::{
    WebSocketStream,
    MaybeTlsStream,
    tungstenite::Message,
};
use tokio::net::TcpStream;
use futures::stream::{SplitSink, SplitStream};

use anyhow::Result;
use serde::{Deserialize, Serialize};

impl GmoWs {
    pub async fn connect_trades(&self, parameters: ConnectTradesParameters) -> Result<(
        SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>,
        SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>
    )> {
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