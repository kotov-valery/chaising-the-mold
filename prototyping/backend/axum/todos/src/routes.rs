use crate::models::Todo;
use crate::state::AppState;

use std::sync::{Arc, Mutex};

use axum::extract::{Path, State};
use axum::{http::StatusCode, response::IntoResponse, Json};

pub async fn list_todos(State(state): State<Arc<Mutex<AppState>>>) -> String {
    let state = state.lock().unwrap();
    serde_json::to_string(&state.storage).unwrap_or_else(|err| {
        error!("Failed to return todo list: {}", err);
        "Error".to_string()
    })
}

pub async fn create_todo(
    State(state): State<Arc<Mutex<AppState>>>,
    Json(input): Json<Todo>,
) -> impl IntoResponse {
    let create = Todo {
        id: input.id,
        description: input.description,
        completed: input.completed,
    };
    let mut state = state.lock().unwrap();
    state.storage.push(create.clone());
    (StatusCode::CREATED, Json(create))
}

pub async fn delete_todo(
    Path(id): Path<u64>,
    State(state): State<Arc<Mutex<AppState>>>,
) -> impl IntoResponse {
    let mut state = state.lock().unwrap();
    let count = state.storage.len();
    state.storage.retain(|todo| todo.id != id);
    if state.storage.len() != count {
        (
            StatusCode::NO_CONTENT,
            format!("Item with id {} was deleted", id),
        )
    } else {
        (StatusCode::NOT_FOUND, format!("Did not find {} item", id))
    }
}

pub async fn update_todo(
    Path(todo_id): Path<u64>,
    State(state): State<Arc<Mutex<AppState>>>,
    Json(input): Json<Todo>,
) -> Result<impl IntoResponse, StatusCode> {
    let update = Todo {
        id: input.id,
        description: input.description,
        completed: input.completed,
    };
    let mut state = state.lock().unwrap();
    for todo in state.storage.iter_mut() {
        if todo.id == todo_id {
            *todo = update.clone();
            break;
        }
    }
    Ok(Json(update))
}
