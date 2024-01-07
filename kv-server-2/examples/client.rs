use anyhow::Result;
use async_prost::AsyncProstStream;
use futures::prelude::*;
use kv_server_2::pb::{
    command_request,
    value,
    CommandRequest,
    Hset,
    // KvError,
    KvPair,
    Value,
};
use prost::Message;
use tokio::net::TcpStream;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let addr = "localhost:9527";

    let stream = TcpStream::connect(addr).await?;

    let mut stream =
        AsyncProstStream::<tokio::net::TcpStream, Vec<u8>, Vec<u8>, _>::from(stream).for_async();

    let msg = CommandRequest {
        request_data: Some(command_request::RequestData::Hset(Hset {
            table: "table1".to_string(),
            pair: Some(KvPair {
                key: "hello".into(),
                value: Some(Value {
                    value: Some(value::Value::String("world".into())),
                }),
            }),
        })),
    };
    stream.send(msg.encode_to_vec()).await?;

    // let msg = Request::new_get("hello");
    // stream.send(msg.into()).await?;

    // while let Some(Result::Ok(buf)) = stream.next().await {
    //     let msg: Response = Response::try_from(buf)?;
    //     info!("Response: {:?}", msg);
    // }

    Ok(())
}
