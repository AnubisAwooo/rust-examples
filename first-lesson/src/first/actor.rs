use anyhow::Result;
use tokio::sync::{mpsc, oneshot};

struct Actor<State, Request, Reply> {
    receiver: mpsc::Receiver<ActorMessage<Request, Reply>>,
    state: State,
}

impl<State, Request, Reply> Actor<State, Request, Reply>
where
    State: Default + Send + 'static,
    Request: HandleCall<State = State, Reply = Reply> + Send + 'static,
    Reply: Send + 'static,
{
    fn spawn(mailbox: usize) -> Pid<Request, Reply> {
        let (sender, receiver) = mpsc::channel(mailbox);
        let mut actor: Actor<State, Request, Reply> = Actor {
            receiver,
            state: State::default(),
        };

        tokio::spawn(async move {
            while let Some(message) = actor.receiver.recv().await {
                let reply = message.data.handle_call(&mut actor.state).unwrap();
                let _ = message.sender.send(reply);
            }
        });

        Pid { sender }
    }
}

struct ActorMessage<Request, Reply> {
    data: Request,
    sender: oneshot::Sender<Reply>,
}

trait HandleCall {
    type State;
    type Reply;
    fn handle_call(&self, state: &mut Self::State) -> Result<Self::Reply>;
}

#[derive(Clone)]
struct Pid<Request, Reply> {
    sender: mpsc::Sender<ActorMessage<Request, Reply>>,
}

impl<Request, Reply> Pid<Request, Reply> {
    pub async fn send(&self, data: Request) -> Result<Reply> {
        let (sender, receiver) = oneshot::channel();
        let _ = self.sender.send(ActorMessage { data, sender }).await;
        Ok(receiver.await?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test() {
        impl HandleCall for usize {
            type State = usize;

            type Reply = usize;

            fn handle_call(&self, state: &mut Self::State) -> Result<Self::Reply> {
                let s = *state;
                *state = *self;
                Ok(s)
            }
        }

        let pid: Pid<usize, usize> = Actor::spawn(20);
        let r = pid.send(1).await.unwrap();
        assert_eq!(r, 0);
        let r = pid.send(1).await.unwrap();
        assert_eq!(r, 1);

        let pid2 = pid.clone();
        let r = pid2.send(3).await.unwrap();
        assert_eq!(r, 1);

        let pid2 = pid.clone();
        let r = pid2.send(4).await.unwrap();
        assert_eq!(r, 3);
    }
}
