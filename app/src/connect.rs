use serde::Serialize;
use anyhow::Result;
use tokio_tungstenite::{
    WebSocketStream,
    MaybeTlsStream,
    tungstenite::Message,
};
use tokio::net::TcpStream;
use futures::{
    Future,
    stream::{SplitSink, SplitStream},
};

pub trait Connect {
    fn connect(&self) -> impl Future<Output = Result<(
        SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>,
        SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>
    )>>;
    fn execute<T: Serialize>(&self, parameters: T) -> impl Future <Output = Result<(
        SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>,
        SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>
    )>>;
}