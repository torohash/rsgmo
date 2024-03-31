use serde::Serialize;
use anyhow::Result;
use futures::Future;

pub trait Connect {
    // fn connect(&self) -> impl Future<Output = Result<(
    //     SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>,
    //     SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>
    // )>>;
    fn execute<T: Serialize>(&self, parameters: T) -> impl Future <Output = Result<()>>;
}