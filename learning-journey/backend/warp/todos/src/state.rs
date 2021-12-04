use crate::models::{self, Todo, ListOptions, Storage};

use tokio::sync::{mpsc, oneshot};

pub enum Message {
    List {
        options: ListOptions,
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

pub struct State {
    rx: Receiver,
    storage: Storage,
}

impl State {
    pub fn new(rx: Receiver) -> Self {
        State { 
            rx: rx,
            storage: models::create_empty_storage(),
        }
    }

    #[cfg(test)]
    pub fn test_add_todo(&mut self, todo: Todo) {
        self.storage.push(todo);
    }

    pub async fn run(&mut self) {
        while let Some(message) = self.rx.recv().await {
            use Message::*;
            match message {
                List { options, resp } => {
                    let list = self.storage
                        .clone()
                        .into_iter()
                        .skip(options.offset.unwrap_or(0))
                        .take(options.limit.unwrap_or(std::usize::MAX))
                        .collect();
                    let _ = resp.send(Some(list));
                },
                Create { create, resp } => {
                    let mut status = Status::Created;
                    for todo in self.storage.iter() {
                        if todo.id == create.id {
                            log::debug!("\t\t-> todo with {} id already exists", create.id);
                            status = Status::Duplicate;
                            break;
                        }
                    }
                    if status == Status::Created {
                        self.storage.push(create);
                    }
                    let _ = resp.send(Some(status));
                },
                Delete { id, resp } => {
                    let count = self.storage.len();
                    // Retain all todo items with a different id,
                    // meaning delete todos with the provided id
                    self.storage.retain(|todo| todo.id != id);

                    let deleted = self.storage.len() != count;
                    let mut status = Status::Deleted;
                    if !deleted {
                        status = Status::NotFound;
                    }
                    let _ = resp.send(Some(status));
                },
                Update { id, update, resp } => {
                    let mut status = Status::NotFound;
                    for todo in self.storage.iter_mut() {
                        if todo.id == id {
                            *todo = update;
                            status = Status::Updated;
                            break;
                        }
                    }
                    let _ = resp.send(Some(status));
                },
            }
        }
    }
}