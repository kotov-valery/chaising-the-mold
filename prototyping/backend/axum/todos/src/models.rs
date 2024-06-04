use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Todo {
    pub id: u64,
    pub description: String,
    pub completed: bool,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Pagination {
    pub offset: Option<usize>,
    pub limit: Option<usize>,
}

impl Todo {
    pub fn new(id: u64, description: &str, completed: bool) -> Self {
        Self {
            id: id,
            description: description.to_string(),
            completed: completed,
        }
    }
}

pub type Storage = Vec<Todo>;
pub fn create_storage() -> Storage {
    vec![
        Todo::new(1, "entry 1", false),
        Todo::new(2, "entry 2", true),
    ]
}
