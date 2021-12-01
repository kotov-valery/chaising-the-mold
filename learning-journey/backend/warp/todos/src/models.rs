use serde_derive::{Serialize, Deserialize};
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Todo {
    pub id: u64,
    pub description: String,
    pub completed: bool,
}

pub type Storage = Arc<Mutex<Vec<Todo>>>;

pub fn create_empty_storage() -> Storage {
    Arc::new(Mutex::new(Vec::new()))
}