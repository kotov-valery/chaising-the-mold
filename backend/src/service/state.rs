use crate::service::models::Measurement;

use tokio::sync::{mpsc, oneshot};

use crate::storage::Storage;

type Receiver = mpsc::Receiver<Message>;
pub type Sender = mpsc::Sender<Message>;
pub type Responder<T> = oneshot::Sender<Option<T>>;

pub enum Message {
    Get { resp: Responder<Vec<Measurement>> },
}

pub struct State {
    rx: Receiver,
    storage: Box<dyn Storage + Send>,
}

impl State {
    pub fn new(rx: Receiver, storage: Box<dyn Storage + Send>) -> Self {
        State { rx, storage }
    }

    pub async fn run(&mut self) {
        while let Some(message) = self.rx.recv().await {
            match message {
                Message::Get { resp } => {
                    let measurements = self
                        .storage
                        .read()
                        .iter()
                        .map(|data| Measurement::from(data))
                        .collect();
                    let _ = resp.send(Some(measurements));
                }
            }
        }
    }
}
