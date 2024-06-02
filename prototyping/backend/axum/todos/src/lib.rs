mod models;
mod routes;
mod state;

use std::error::Error;
#[macro_use]
extern crate log;

use axum::routing::{delete, get, post, put};
use axum::Router;

use std::net::{Ipv4Addr, SocketAddrV4};
use std::sync::{Arc, Mutex};

pub async fn start_web_server(host_addr: &str, port_number: u16) -> Result<(), Box<dyn Error>> {
    info!(
        "Starting a web service on {} address on {} port...",
        host_addr, port_number
    );

    let state = Arc::new(Mutex::new(state::AppState::new()));
    let app = Router::new()
        .route("/todos", get(routes::list_todos))
        .route("/todos", post(routes::create_todo))
        .route("/todos/:id", delete(routes::delete_todo))
        .route("/todos/:id", put(routes::update_todo))
        .with_state(state);

    let addr: Ipv4Addr = host_addr.parse()?;
    let listener = tokio::net::TcpListener::bind(SocketAddrV4::new(addr, port_number)).await?;
    Ok(axum::serve(listener, app).await?)
}
