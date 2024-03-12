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
    pub async fn connect_orderbooks(&self, parameters: ConnectOrderbooksParameters) -> Result<(
        SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>,
        SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>
    )> {
        self.execute(parameters).await
    }
}


#[derive(Debug, Clone, Serialize)]
pub struct ConnectOrderbooksParameters {
    command: CommandType,
    channel: Channel,
    symbol: String,
}

impl ConnectOrderbooksParameters {
    pub fn new(command: CommandType, symbol: &str) -> Self {
        Self {
            command,
            channel: Channel::Orderbooks,
            symbol: symbol.to_string(),
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConnectOrderbooksResponse {
    channel: String,
    asks: Vec<Order>,
    bids: Vec<Order>,
    symbol: String,
    timestamp: String,
}

impl ConnectOrderbooksResponse {
    pub fn channel(&self) -> &str {
        &self.channel
    }
    pub fn asks(&self) -> &Vec<Order> {
        &self.asks
    }
    pub fn bids(&self) -> &Vec<Order> {
        &self.bids
    }
    pub fn symbol(&self) -> &str {
        &self.symbol
    }
    pub fn timestamp(&self) -> &str {
        &self.timestamp
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Order {
    #[serde(deserialize_with = "utils::deserialize_string_to_u64")]
    price: u64,
    #[serde(deserialize_with = "utils::deserialize_f64")]
    size: f64,
}

impl Order {
    pub fn price(&self) -> u64 {
        self.price
    }
    pub fn size(&self) -> f64 {
        self.size
    }
}