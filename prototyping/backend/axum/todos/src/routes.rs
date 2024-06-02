use crate::models::Todo;
use crate::state::AppState;

use std::sync::{Arc, Mutex};

use axum::extract::{Path, State};
use axum::Json;

pub async fn list_todos(State(state): State<Arc<Mutex<AppState>>>) -> String {
    let state = state.lock().unwrap();
    serde_json::to_string(&state.storage).unwrap_or_else(|err| {
        error!("Failed to return todo list: {}", err);
        "Error".to_string()
    })
}

pub async fn create_todo(State(state): State<Arc<Mutex<AppState>>>, Json(input): Json<Todo>) -> String {
    let todo = Todo {
        id: input.id,
        description: input.description,
        completed: input.completed,
    };
    info!("Got a todo item to create: {:?}", todo);
    let mut state = state.lock().unwrap();
    state.storage.push(todo);
    "Created".to_string()
}

pub async fn delete_todo(Path(todo_id): Path<u64>, State(state): State<Arc<Mutex<AppState>>>) -> String {
    info!("Todo id: {}", todo_id);
    let mut state = state.lock().unwrap();
    state.storage.retain(|todo| todo.id != todo_id);
    "Deleted".to_string()
}

pub async fn update_todo(
    Path(todo_id): Path<u64>,
    State(state): State<Arc<Mutex<AppState>>>,
    Json(input): Json<Todo>,
) -> String {
    let update = Todo {
        id: input.id,
        description: input.description,
        completed: input.completed,
    };
    info!("Todo id {} and content {:?}", todo_id, update);
    let mut state = state.lock().unwrap();
    for todo in state.storage.iter_mut() {
        if todo.id == todo_id {
            *todo = update;
            break;
        }
    }
    "Updated".to_string()
}
