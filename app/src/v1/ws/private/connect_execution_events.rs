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
    pub async fn connect_execution_events(&self, parameters: ConnectExecutionEventsParameters) -> Result<(
        SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>,
        SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>
    )> {
        self.execute(parameters).await
    }
}


#[derive(Debug, Clone, Serialize)]
pub struct ConnectExecutionEventsParameters {
    command: CommandType,
    channel: Channel,
}

impl ConnectExecutionEventsParameters {
    pub fn new(command: CommandType) -> Self {
        Self {
            command,
            channel: Channel::ExecutionEvents,
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConnectExecutionEventsResponse {
    channel: String,
    order_id: i64,
    execution_id: i64,
    symbol: String,
    settle_type: String,
    execution_type: String,
    side: String,
    #[serde(deserialize_with = "utils::deserialize_f64")]
    execution_price: f64,
    #[serde(deserialize_with = "utils::deserialize_f64")]
    execution_size: f64,
    position_id: Option<i64>,
    order_timestamp: String,
    execution_timestamp: String,
    #[serde(deserialize_with = "utils::deserialize_f64")]
    loss_gain: f64,
    #[serde(deserialize_with = "utils::deserialize_f64")]
    fee: f64,
    #[serde(deserialize_with = "utils::deserialize_f64")]
    order_price: f64,
    #[serde(deserialize_with = "utils::deserialize_f64")]
    order_size: f64,
    #[serde(deserialize_with = "utils::deserialize_f64")]
    order_executed_size: f64,
    time_in_force: String,
    msg_type: String,
}

impl ConnectExecutionEventsResponse {
    pub fn channel(&self) -> &str {
        &self.channel
    }
    pub fn order_id(&self) -> i64 {
        self.order_id
    }
    pub fn execution_id(&self) -> i64 {
        self.execution_id
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
    pub fn execution_price(&self) -> f64 {
        self.execution_price
    }
    pub fn execution_size(&self) -> f64 {
        self.execution_size
    }
    pub fn position_id(&self) -> Option<i64> {
        self.position_id
    }
    pub fn order_timestamp(&self) -> &str {
        &self.order_timestamp
    }
    pub fn execution_timestamp(&self) -> &str {
        &self.execution_timestamp
    }
    pub fn loss_gain(&self) -> f64 {
        self.loss_gain
    }
    pub fn fee(&self) -> f64 {
        self.fee
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
    pub fn time_in_force(&self) -> &str {
        &self.time_in_force
    }
    pub fn msg_type(&self) -> &str {
        &self.msg_type
    }
}