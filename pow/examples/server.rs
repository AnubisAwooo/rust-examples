use std::{collections::HashMap, pin::Pin, sync::Arc, thread};

use tokio::sync::{mpsc, RwLock};

use anyhow::Result;
use futures::Stream;
use pow::{
    pb::{pow_builder_server::*, *},
    pow::do_pow,
};
use tokio_stream::wrappers::ReceiverStream;
use tonic::{Response, Status};
use tracing::info;

const CHANNEL_SIZE: usize = 8;

#[derive(Debug)]
struct Shared {
    clients: HashMap<String, mpsc::Sender<Result<BlockHash, Status>>>,
}

impl Shared {
    fn new() -> Self {
        Self {
            clients: HashMap::new(),
        }
    }
    async fn broadcast(&self, msg: Option<BlockHash>) {
        let msg = msg.ok_or(Status::resource_exhausted(
            "failed to find a suitable hash.",
        ));
        for (name, tx) in &self.clients {
            match tx.send(msg.clone()).await {
                Ok(_) => {}
                Err(err) => {
                    println!(
                        "Broadcast error to {} with {:?}. Error: {:?}",
                        name, msg, err
                    )
                }
            };
        }
    }
}

#[derive(Debug)]
struct PowService {
    tx: mpsc::Sender<Block>,
    shared: Arc<RwLock<Shared>>,
}

#[tonic::async_trait]
impl PowBuilder for PowService {
    type SubscribeStream = Pin<Box<dyn Stream<Item = Result<BlockHash, Status>> + Send + Sync>>;

    #[must_use]
    #[allow(clippy::type_complexity, clippy::type_repetition_in_bounds)]
    async fn subscribe(
        &self,
        request: tonic::Request<ClientInfo>,
    ) -> std::result::Result<tonic::Response<Self::SubscribeStream>, tonic::Status> {
        let name = request.into_inner().name;

        let rx = {
            let (tx, rx) = mpsc::channel(CHANNEL_SIZE);
            self.shared.write().await.clients.insert(name, tx);
            rx
        };

        Ok(Response::new(Box::pin(ReceiverStream::new(rx))))
    }

    #[must_use]
    #[allow(clippy::type_complexity, clippy::type_repetition_in_bounds)]
    async fn submit(
        &self,
        request: tonic::Request<Block>,
    ) -> std::result::Result<tonic::Response<BlockStatus>, tonic::Status> {
        let block = request.into_inner();
        match self.tx.send(block.clone()).await {
            Ok(()) => Ok(Response::new(BlockStatus { code: 0 })),
            Err(err) => {
                println!(
                    "Failed to submit {:?} to Pow engine. Error: {:?}",
                    block, err
                );
                Ok(Response::new(BlockStatus { code: 500 }))
            }
        }
    }
}

impl PowService {
    fn new(tx: mpsc::Sender<Block>, mut rx: mpsc::Receiver<Option<BlockHash>>) -> Self {
        let service = Self {
            tx,
            shared: Arc::new(RwLock::new(Shared::new())),
        };

        let shared = service.shared.clone();

        tokio::spawn(async move {
            while let Some(msg) = rx.recv().await {
                shared.read().await.broadcast(msg).await
            }
        });

        service
    }
}

async fn start_server(addr: &str) -> Result<()> {
    // grpc -> pow
    let (tx1, mut rx1) = mpsc::channel::<Block>(CHANNEL_SIZE);
    // pow -> grpc
    let (tx2, rx2) = mpsc::channel::<Option<BlockHash>>(CHANNEL_SIZE);

    thread::spawn(move || {
        while let Some(block) = rx1.blocking_recv() {
            let result = do_pow(block);
            tx2.blocking_send(result).unwrap();
        }
    });

    let service = PowService::new(tx1, rx2);

    let addr = addr.parse()?;
    tonic::transport::Server::builder()
        .add_service(PowBuilderServer::new(service))
        .serve(addr)
        .await?;

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let addr = "0.0.0.0:8888";
    info!("Listening to: {}", addr);

    start_server(addr).await?;

    Ok(())
}
