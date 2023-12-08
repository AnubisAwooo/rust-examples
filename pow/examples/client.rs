use anyhow::Result;
use pow::pb::{pow_builder_client::*, *};
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let addr = "http://localhost:8888";

    let mut client = PowBuilderClient::connect(addr).await?;

    let mut stream = client
        .subscribe(ClientInfo {
            name: "client".into(),
        })
        .await?
        .into_inner();

    let res = client
        .submit(Block {
            data: b"hello world".into(),
            ..Default::default()
        })
        .await?
        .into_inner();

    info!("Submitted: {:?}", res);

    let mut count = 0;

    while let Some(result) = stream.message().await? {
        // format!("{}", String::from_utf8(result.data))
        info!(
            "Received: data: {} nonce: {} hash: 0x{}",
            String::from_utf8(result.data).unwrap(),
            result.nonce,
            hex::encode(result.hash)
        );
        count += 1;
        if count < 4 {
            client
                .submit(Block {
                    data: format!("hello world: {}", count).into_bytes(),
                    difficult: count,
                })
                .await?
                .into_inner();
        } else {
            println!("done")
        }
    }

    Ok(())
}
