use anyhow::Result;
use async_prost::AsyncProstStream;
use futures::prelude::*;
use kv_server_4::{CommandRequest, MemTable, Service, ServiceInner};
use prost::Message;
use tokio::net::TcpListener;
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let addr = "127.0.0.1:9527";
    let listener = TcpListener::bind(addr).await?;
    info!("Start listening on {}", addr);

    // main 函数开头，初始化 service
    let service: Service = ServiceInner::new(MemTable::new()).into();
    // while loop 中，使用 svc 来执行 cmd
    loop {
        let (stream, addr) = listener.accept().await?;
        info!("Client {:?} connected", addr);

        // tokio::spawn 之前，复制一份 service
        let svc = service.clone();

        tokio::spawn(async move {
            let mut stream = AsyncProstStream::<_, Vec<u8>, Vec<u8>, _>::from(stream).for_async();

            while let Some(Ok(msg)) = stream.next().await {
                let msg = CommandRequest::decode(&msg[..]).unwrap();
                info!("Got a new command: {:?}", msg);
                let res = svc.execute(msg);
                stream.send(res.encode_to_vec()).await.unwrap();

                svc.after_send();
            }
            info!("Client {:?} disconnected", addr);
        });
    }
}
