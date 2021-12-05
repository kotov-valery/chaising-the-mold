
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Todo {
    pub id: u64,
    pub description: String,
    pub completed: bool,
}

#[derive(Debug)]
pub struct ListOptions {
    pub offset: Option<usize>,
    pub limit: Option<usize>,
}

pub type Storage = Vec<Todo>;

pub fn create_empty_storage() -> Storage {
    Vec::new()
}