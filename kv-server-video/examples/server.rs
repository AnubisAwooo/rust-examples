use std::sync::Arc;

use anyhow::{Ok, Result};
use dashmap::DashMap;
use futures::SinkExt;
use futures::StreamExt;

use kv::pb::{request::Command, Request, RequestGet, RequestPut, Response};
use tokio::net::TcpListener;
use tokio_util::codec::LengthDelimitedCodec;
use tracing::info;

#[derive(Debug, Default)]
struct ServerState {
    state: DashMap<String, Vec<u8>>,
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let state = Arc::new(ServerState::default());
    let addr = "0.0.0.0:8888";
    let listener = TcpListener::bind(addr).await?;
    info!("Listening to: {}", addr);

    loop {
        let (stream, addr) = listener.accept().await?;
        info!("new client: {:?} accepted", addr);
        let shared = state.clone();
        tokio::spawn(async move {
            let mut stream = LengthDelimitedCodec::builder()
                .length_field_length(2)
                .new_framed(stream);
            while let Some(Result::Ok(buf)) = stream.next().await {
                let msg: Request = buf.try_into()?;
                info!("Got a command: {:?}", msg);

                let response = match msg.command {
                    Some(Command::Get(RequestGet { key })) => match shared.state.get(&key) {
                        Some(v) => Response::new(key, v.value().to_vec()),
                        None => Response::not_found(key),
                    },
                    Some(Command::Put(RequestPut { key, value })) => {
                        let old = shared.state.insert(key.clone(), value);
                        Response::new(key, old.unwrap_or_default())
                    }
                    _ => unimplemented!(),
                };

                stream.send(response.into()).await?
            }
            Ok(())
        });
    }
}
