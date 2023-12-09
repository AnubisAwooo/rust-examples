use anyhow::{Ok, Result};
use bytes::Bytes;
use futures::SinkExt;
use futures::StreamExt;

use kv::pb::{Request, Response};
use tokio::net::TcpStream;
// use tokio_util::codec::LengthDelimitedCodec;
use kv::noise_codec::{NoiseCodec, NOISE_PARAMS};
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let addr = "localhost:8888";

    let stream = TcpStream::connect(addr).await?;

    // let mut stream = LengthDelimitedCodec::builder()
    //     .length_field_length(2)
    //     .new_framed(stream);
    let mut stream = NoiseCodec::builder(NOISE_PARAMS, true).new_framed(stream)?;
    stream.send(Bytes::from_static(&[])).await?;
    info!("-> e");
    let data = stream.next().await.unwrap()?;
    info!("<- e, ee, s, es");
    stream.send(data.freeze()).await?;
    info!("-> s, se");
    stream.codec_mut().into_transport_mode()?;

    let msg = Request::new_put("hello", b"world");
    stream.send(msg.into()).await?;

    let msg = Request::new_get("hello");
    stream.send(msg.into()).await?;

    let msg = Request::new_put("hello", b"hello world");
    stream.send(msg.into()).await?;

    let msg = Request::new_get("hello");
    stream.send(msg.into()).await?;

    let msg = Request::new_delete("hello");
    stream.send(msg.into()).await?;

    let msg = Request::new_get("hello");
    stream.send(msg.into()).await?;

    while let Some(Result::Ok(buf)) = stream.next().await {
        let msg: Response = Response::try_from(buf)?;
        info!("Response: {:?}", msg);
    }

    Ok(())
}
