use serde_derive::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Todo {
    pub id: u64,
    pub description: String,
    pub completed: bool,
}

pub type Storage = Vec<Todo>;

pub fn create_empty_storage() -> Storage {
    Vec::new()
}