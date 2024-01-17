use anyhow::Result;
use futures::prelude::*;
use kv_server_7::{CommandRequest, CommandResponse};
use prost::Message;
use tokio::net::TcpStream;
use tokio_util::codec::{Framed, LengthDelimitedCodec};
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let addr = "127.0.0.1:9527";
    // 连接服务器
    let stream = TcpStream::connect(addr).await?;

    // 使用 AsyncProstStream 来处理 TCP Frame
    let mut client = Framed::new(stream, LengthDelimitedCodec::new());

    // 生成一个 HSET 命令
    let cmd = CommandRequest::new_hset("table1", "hello", "world".into());

    // 发送 HSET 命令
    client.send(cmd.encode_to_vec().into()).await?;
    if let Some(Ok(data)) = client.next().await {
        let data: CommandResponse = CommandResponse::decode(&data[..]).unwrap();
        info!("Got response {:?}", data);

        client.send(cmd.encode_to_vec().into()).await?;
        if let Some(Ok(data)) = client.next().await {
            let data: CommandResponse = CommandResponse::decode(&data[..]).unwrap();
            info!("Got response {:?}", data);
        }
    }

    Ok(())
}
