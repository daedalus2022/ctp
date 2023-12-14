use std::sync::Arc;

use async_prost::AsyncProstStream;
use ctp::{CommandRequest, CommandResponse, CtpCommand, CtpService, CtpSys, Service};
use ctp_sys::CtpAccountConfig;
use futures::{SinkExt, StreamExt};
use tokio::{
    net::TcpListener,
    sync::{mpsc, Mutex},
};
use tracing::info;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let account = CtpAccountConfig {
        broker_id: "9999".to_string(),
        account: "15801632955".to_string(),
        trade_front: "tcp://180.168.146.187:10201".to_string(),
        // md_front: "tcp://180.168.146.187:10131".to_string(),
        md_front: "tcp://180.168.146.187:10211".to_string(),
        name_server: "".to_string(),
        auth_code: "0000000000000000".to_string(),
        user_product_info: "".to_string(),
        app_id: "simnow_client_test".to_string(),
        password: "87406037".to_string(),
    };

    info!("完成保存查询结果");

    let (ctp_sender, ctp_sys_receiver) = mpsc::unbounded_channel::<CtpCommand>();

    let ctp = CtpSys::new(
        account,
        (ctp_sender, Arc::new(Mutex::new(ctp_sys_receiver))),
    );

    ctp.init();

    let service = Service::new(ctp);

    let addr = "127.0.0.1:10130";

    let listener = TcpListener::bind(addr).await?;

    info!("Start listening on {}", addr);

    loop {
        let (stream, addr) = listener.accept().await?;
        info!("Client {:?} connected", addr);

        let svc = service.clone();

        tokio::spawn(async move {
            let mut stream =
                AsyncProstStream::<_, CommandRequest, CommandResponse, _>::from(stream).for_async();
            while let Some(Ok(cmd)) = stream.next().await {
                let res = svc.execute(cmd);
                stream.send(res).await.unwrap();
            }
            info!("Client {:?} disconnected", addr);
        });
    }
}
