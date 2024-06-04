use crate::models::{self, Pagination, Storage, Todo};

use tokio::sync::{mpsc, oneshot};

pub enum Message {
    List {
        options: Pagination,
        resp: Responder<Vec<Todo>>,
    },
    Create {
        create: Todo,
        resp: Responder<Status>,
    },
    Delete {
        id: u64,
        resp: Responder<Status>,
    },
    Update {
        id: u64,
        update: Todo,
        resp: Responder<Status>,
    },
}

#[derive(PartialEq, Eq)]
pub enum Status {
    Created,
    Deleted,
    Updated,
    Duplicate,
    NotFound,
}

type Receiver = mpsc::Receiver<Message>;
pub type Sender = mpsc::Sender<Message>;
pub type Responder<T> = oneshot::Sender<Option<T>>;

pub struct AppState {
    rx: Receiver,
    storage: Storage,
}

impl AppState {
    pub fn new(rx: Receiver) -> Self {
        Self {
            rx: rx,
            storage: models::create_storage(),
        }
    }

    pub async fn run(&mut self) {
        while let Some(message) = self.rx.recv().await {
            match message {
                Message::List { options, resp } => {
                    let list = self
                        .storage
                        .clone()
                        .into_iter()
                        .skip(options.offset.unwrap_or(0))
                        .take(options.limit.unwrap_or(std::usize::MAX))
                        .collect();
                    let _ = resp.send(Some(list));
                }
                Message::Create { create, resp } => {
                    let mut status = Status::Created;
                    for todo in self.storage.iter() {
                        if todo.id == create.id {
                            log::debug!("Todo with {} id already exists", create.id);
                            status = Status::Duplicate;
                            break;
                        }
                    }
                    if status == Status::Created {
                        self.storage.push(create);
                    }
                    let _ = resp.send(Some(status));
                }
                Message::Delete { id, resp } => {
                    let count = self.storage.len();
                    self.storage.retain(|todo| todo.id != id);

                    let deleted = self.storage.len() != count;
                    let mut status = Status::Deleted;
                    if !deleted {
                        status = Status::NotFound;
                    }
                    let _ = resp.send(Some(status));
                }
                Message::Update { id, update, resp } => {
                    let mut status = Status::NotFound;
                    for todo in self.storage.iter_mut() {
                        if todo.id == id {
                            *todo = update;
                            status = Status::Updated;
                            break;
                        }
                    }
                    let _ = resp.send(Some(status));
                }
            }
        }
    }
}
