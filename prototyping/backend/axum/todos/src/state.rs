use crate::models::{self, Pagination, Storage, Todo};

use tokio::sync::{mpsc, oneshot};
use uuid::Uuid;

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
        id: Uuid,
        resp: Responder<Status>,
    },
    Update {
        id: Uuid,
        description: Option<String>,
        completed: Option<bool>,
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
                        .values()
                        .skip(options.offset.unwrap_or(0))
                        .take(options.limit.unwrap_or(std::usize::MAX))
                        .cloned()
                        .collect::<Vec<_>>();
                    let _ = resp.send(Some(list));
                }
                Message::Create { create, resp } => {
                    if self.storage.contains_key(&create.id) {
                        let _ = resp.send(Some(Status::Duplicate));
                        return;
                    }
                    self.storage.insert(create.id, create);
                    let _ = resp.send(Some(Status::Created));
                }
                Message::Delete { id, resp } => {
                    if !self.storage.contains_key(&id) {
                        let _ = resp.send(Some(Status::NotFound));
                        return;
                    }
                    self.storage.remove(&id);
                    let _ = resp.send(Some(Status::Deleted));
                }
                Message::Update { id, description, completed, resp } => {
                    if !self.storage.contains_key(&id) {
                        let _ = resp.send(Some(Status::NotFound));
                        return;
                    }
                    let mut todo = self.storage.get(&id).cloned().unwrap();
                    if let Some(description) = description {
                        todo.description = description;
                    }
                    if let Some(completed) = completed {
                        todo.completed = completed;
                    }
                    self.storage.insert(id, todo.clone());
                    let _ = resp.send(Some(Status::Updated));
                }
            }
        }
    }
}
