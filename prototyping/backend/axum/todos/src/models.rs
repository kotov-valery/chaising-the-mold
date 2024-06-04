use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Todo {
    pub id: Uuid,
    pub description: String,
    pub completed: bool,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Pagination {
    pub offset: Option<usize>,
    pub limit: Option<usize>,
}

#[derive(Debug, Deserialize)]
pub struct CreateTodo {
    pub description: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateTodo {
    pub description: Option<String>,
    pub completed: Option<bool>,
}

pub type Storage = HashMap<Uuid, Todo>;
pub fn create_storage() -> Storage {
    HashMap::new()
}
