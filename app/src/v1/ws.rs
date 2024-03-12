pub mod public;
pub mod private;

use core::fmt;

use crate::{
    connect::Connect, constants::{V1_PATH, WS_BASE_URL}, request::AccessLevel
};
use serde::Serialize;
use anyhow::Result;
use tokio_tungstenite::{
    connect_async,
    WebSocketStream,
    MaybeTlsStream,
    tungstenite::Message,
};
use tokio::net::TcpStream;
use futures::{
    StreamExt,
    SinkExt,
    stream::{SplitSink, SplitStream},
};

#[derive(Debug, Clone)]
pub struct GmoWs {
    base_url: String,
    access_token: Option<String>,
    access_level: AccessLevel,
}

impl GmoWs {
    pub fn new(access_level: AccessLevel) -> Self {
        Self {
            base_url: WS_BASE_URL.to_string(),
            access_token: None,
            access_level,
        }
    }

    pub fn with_access_token(mut self, access_token:& str) -> Self {
        self.access_token = Some(access_token.to_string());
        self
    }
}

impl Connect for GmoWs {
    async fn connect(&self) -> Result<(
        SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>,
        SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>
    )> {
        let url = match self.access_level {
            AccessLevel::Public => format!("{}{}{}", self.base_url, self.access_level, V1_PATH),
            AccessLevel::Private => {
                match &self.access_token {
                    Some(token) => format!("{}{}{}/{}", self.base_url, self.access_level, V1_PATH, token),
                    None => return Err(anyhow::anyhow!("Access token is required for private web socket connections.")),
                }
            }
        };
        let (ws_stream, _) = connect_async(url).await?;
        Ok(ws_stream.split())
    }

    async fn execute<T: Serialize>(&self, parameters: T) -> Result<(
        SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>,
        SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>
    )> {
        let (mut write, read) = self.connect().await?;
        let message = Message::Text(serde_json::to_string(&parameters)?);
        write.send(message).await?;
        Ok((write, read))
    }
}

#[derive(Debug, Clone, Serialize)]
pub enum CommandType {
    #[serde(rename = "subscribe")]
    Subscribe,
    #[serde(rename = "unsubscribe")]
    Unsubscribe,
}

impl fmt::Display for CommandType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CommandType::Subscribe => write!(f, "subscribe"),
            CommandType::Unsubscribe => write!(f, "unsubscribe"),
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub enum Channel {
    #[serde(rename = "ticker")]
    Ticker,
    #[serde(rename = "orderbooks")]
    Orderbooks,
    #[serde(rename = "trades")]
    Trades,
    #[serde(rename = "executionEvents")]
    ExecutionEvents,
    #[serde(rename = "orderEvents")]
    OrderEvents,
    #[serde(rename = "positionEvents")]
    PositionEvents,
    #[serde(rename = "positionSummaryEvents")]
    PositionSummaryEvents,
}

impl fmt::Display for Channel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Channel::Ticker => write!(f, "ticker"),
            Channel::Orderbooks => write!(f, "orderbooks"),
            Channel::Trades => write!(f, "trades"),
            Channel::ExecutionEvents => write!(f, "executionEvents"),
            Channel::OrderEvents => write!(f, "orderEvents"),
            Channel::PositionEvents => write!(f, "positionEvents"),
            Channel::PositionSummaryEvents => write!(f, "positionSummaryEvents"),
        }
    }
}