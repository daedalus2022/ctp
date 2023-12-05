use async_prost::AsyncProstStream;
use ctp::{CommandRequest, CommandResponse};
use futures::prelude::*;
use tokio::net::TcpStream;
use tracing::info;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let addr = "127.0.0.1:9527";

    let stream = TcpStream::connect(addr).await?;

    let mut client =
        AsyncProstStream::<_, CommandResponse, CommandRequest, _>::from(stream).for_async();

    let command = CommandRequest::new_md_q_version();

    client.send(command).await?;
    if let Some(Ok(data)) = client.next().await {
        info!("Got response:{:?}", data);
    }

    Ok(())
}
