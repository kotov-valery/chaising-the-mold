use crate::state::AppState;

use std::sync::Arc;

use axum::extract::State;

pub async fn list_todos(State(state): State<Arc<AppState>>) -> String {
    serde_json::to_string(&state.storage).unwrap_or_else(|err| {
        error!("Failed to return todo list: {}", err);
        "Error".to_string()
    })
}
