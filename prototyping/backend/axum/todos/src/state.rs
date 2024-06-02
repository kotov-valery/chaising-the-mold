use crate::models::{self, Storage};

pub struct AppState {
    pub storage: Storage,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            storage: models::create_storage(),
        }
    }
}
