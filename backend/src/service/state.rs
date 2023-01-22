use crate::service::models::Measurement;

use tokio::sync::{mpsc, oneshot};

use crate::storage::Storage;

type Receiver = mpsc::Receiver<Message>;
pub type Sender = mpsc::Sender<Message>;
pub type Responder<T> = oneshot::Sender<Option<T>>;

#[derive(Debug)]
pub enum Message {
    Get { resp: Responder<Vec<Measurement>> },
    Stop,
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
                Message::Stop => {
                    return;
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::{Message, State};
    use crate::sensing::sensor::DataPoint;
    use crate::service::models::Measurement;
    use crate::storage::MockStorage;
    use tokio::sync::{mpsc, oneshot};

    #[tokio::test]
    async fn start_and_stop() {
        let (tx, rx) = mpsc::channel(10);
        let storage = Box::new(MockStorage::new());
        let mut state = State::new(rx, storage);
        let state = tokio::spawn(async move {
            state.run().await;
        });
        let _ = tx.send(Message::Stop).await;
        state.await.unwrap();
        assert!(true);
    }

    #[tokio::test]
    async fn read_dummy_data() {
        let (tx, rx) = mpsc::channel(10);

        let measurement_data = vec![DataPoint {
            temperature: 19.6,
            humidity: 65.0,
        }];
        let mut storage = MockStorage::new();
        storage.expect_read().return_const(measurement_data.clone());

        let storage = Box::new(storage);
        let mut state = State::new(rx, storage);
        let state = tokio::spawn(async move {
            state.run().await;
        });

        let (resp_tx, resp_rx) = oneshot::channel();
        let _ = tx.send(Message::Get { resp: resp_tx }).await;

        let expected_result = vec![Measurement {
            temperature: String::from("19.60"),
            humidity: String::from("65.00"),
        }];
        if let Ok(Some(result)) = resp_rx.await {
            assert_eq!(expected_result, result);
        } else {
            panic!("Got wrong result from the state");
        }

        let _ = tx.send(Message::Stop).await;
        state.await.unwrap();
        assert!(true);
    }
}
