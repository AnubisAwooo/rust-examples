use anyhow::Result;
use async_prost::AsyncProstStream;
use bytes::BytesMut;
use dashmap::DashMap;
use futures::prelude::*;
use kv_server_1::pb::{
    command_request::RequestData,
    CommandRequest,
    CommandResponse,
    Hset,
    // KvError,
    KvPair,
    Value,
};
use prost::Message;
use std::sync::Arc;
use tokio::net::TcpListener;
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
    // 初始化日志
    tracing_subscriber::fmt::init();

    let addr = "127.0.0.1:9527";
    let listener = TcpListener::bind(addr).await?;
    info!("Start listening on {}", addr);

    // 使用 DashMap 创建放在内存中的 kv store
    let table: Arc<DashMap<String, Value>> = Arc::new(DashMap::new());

    loop {
        // 得到一个客户端请求
        let (stream, addr) = listener.accept().await?;
        info!("Client {:?} connected", addr);

        // 复制 db，让它在 tokio 任务中可以使用
        let db = table.clone();

        // 创建一个 tokio 任务处理这个客户端
        tokio::spawn(async move {
            // 使用 AsyncProstStream 来处理 TCP Frame
            // Frame: 两字节 frame 长度，后面是 protobuf 二进制
            let mut stream =
                AsyncProstStream::<tokio::net::TcpStream, Vec<u8>, Vec<u8>, _>::from(stream)
                    .for_async();

            // 从 stream 里取下一个消息（拿出来后已经自动 decode 了）
            while let Some(Ok(msg)) = stream.next().await {
                let msg: &[u8] = &msg;
                let msg: BytesMut = msg.into();
                let msg: CommandRequest = msg.try_into().unwrap();
                info!("Got a new command: {:?}", msg);
                let resp: CommandResponse = match msg.request_data {
                    // 为演示我们就处理 HSET
                    Some(RequestData::Hset(cmd)) => hset(cmd, &db),
                    // 其它暂不处理
                    _ => unimplemented!(),
                };

                info!("Got response: {:?}", resp);
                // 把 CommandResponse 发送给客户端
                stream.send(resp.encode_to_vec()).await.unwrap();
            }
        });
    }
}

// 处理 hset 命令
fn hset(cmd: Hset, db: &DashMap<String, Value>) -> CommandResponse {
    match cmd.pair {
        Some(KvPair {
            key,
            value: Some(v),
        }) => {
            // 往 db 里写入
            let old = db.insert(key, v).unwrap_or_default();
            // 把 value 转换成 CommandResponse
            old.into()
        }
        _v => {
            // KvError::InvalidCommand(format!("hset: {:?}", v)).into()
            unimplemented!()
        }
    }
}
