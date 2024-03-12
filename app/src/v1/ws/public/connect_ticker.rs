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
    pub async fn connect_ticker(&self, parameters: ConnectTickerParameters) -> Result<(
        SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>,
        SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>
    )> {
        self.execute(parameters).await
    }
}


#[derive(Debug, Clone, Serialize)]
pub struct ConnectTickerParameters {
    command: CommandType,
    channel: Channel,
    symbol: String,
}

impl ConnectTickerParameters {
    pub fn new(command: CommandType, symbol: &str) -> Self {
        Self {
            command,
            channel: Channel::Ticker,
            symbol: symbol.to_string(),
        }
    }
}
#[derive(Debug, Clone, Deserialize)]
pub struct ConnectTickerResponse {
    channel: String,
    #[serde(deserialize_with = "utils::deserialize_string_to_u64")]
    ask: u64,
    #[serde(deserialize_with = "utils::deserialize_string_to_u64")]
    bid: u64,
    #[serde(deserialize_with = "utils::deserialize_string_to_u64")]
    high: u64,
    #[serde(deserialize_with = "utils::deserialize_string_to_u64")]
    last: u64,
    #[serde(deserialize_with = "utils::deserialize_string_to_u64")]
    low: u64,
    symbol: String,
    timestamp: String,
    #[serde(deserialize_with = "utils::deserialize_f64")]
    volume: f64,
}

impl ConnectTickerResponse {
    pub fn channel(&self) -> &str {
        &self.channel
    }
    pub fn ask(&self) -> u64 {
        self.ask
    }
    pub fn bid(&self) -> u64 {
        self.bid
    }
    pub fn high(&self) -> u64 {
        self.high
    }
    pub fn last(&self) -> u64 {
        self.last
    }
    pub fn low(&self) -> u64 {
        self.low
    }
    pub fn symbol(&self) -> &str {
        &self.symbol
    }
    pub fn timestamp(&self) -> &str {
        &self.timestamp
    }
    pub fn volume(&self) -> f64 {
        self.volume
    }
}