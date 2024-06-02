#[macro_use]
extern crate log;

use axum::{extract::State, routing::get, Router};
use serde::Serialize;
use std::error::Error;
use std::net::{Ipv4Addr, SocketAddrV4};
use std::sync::Arc;

#[derive(Debug, Serialize)]
pub struct Todo {
    pub id: u64,
    pub description: String,
    pub completed: bool,
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

pub struct AppState {
    storage: Storage,
}

impl AppState {
    pub fn new(storage: Storage) -> Self {
        Self { storage }
    }
}

pub async fn start_web_server(host_addr: &str, port_number: u16) -> Result<(), Box<dyn Error>> {
    info!(
        "Starting a web service on {} address on {} port...",
        host_addr, port_number
    );

    let storage = Arc::new(AppState::new(create_storage()));
    let app = Router::new()
        .route("/todos", get(get_todos))
        .with_state(storage);

    let addr: Ipv4Addr = host_addr.parse()?;
    let listener = tokio::net::TcpListener::bind(SocketAddrV4::new(addr, port_number)).await?;
    Ok(axum::serve(listener, app).await?)
}

async fn get_todos(State(state): State<Arc<AppState>>) -> String {
    serde_json::to_string(&state.storage).unwrap_or_else(|err| {
        error!("Failed to return todo list: {}", err);
        "Error".to_string()
    })
}
