mod models;
mod routes;
mod state;

use std::error::Error;
#[macro_use]
extern crate log;

use axum::routing::{delete, get, post, put};
use axum::Router;

use tokio::sync::mpsc;

use std::net::{Ipv4Addr, SocketAddrV4};

const DEFAULT_MESSAGE_CAPACITY: usize = 32;

pub async fn start_web_server(host_addr: &str, port_number: u16) -> Result<(), Box<dyn Error>> {
    info!(
        "Starting a web service on {} address on {} port...",
        host_addr, port_number
    );

    let (tx, rx) = mpsc::channel(DEFAULT_MESSAGE_CAPACITY);

    let mut state = state::AppState::new(rx);
    let state = tokio::spawn(async move {
        state.run().await;
    });

    let app = Router::new()
        .route("/todos", get(routes::list_todos))
        .route("/todos", post(routes::create_todo))
        .route("/todos/:id", delete(routes::delete_todo))
        .route("/todos/:id", put(routes::update_todo))
        .with_state(tx.clone());

    let addr: Ipv4Addr = host_addr.parse()?;
    let listener = tokio::net::TcpListener::bind(SocketAddrV4::new(addr, port_number)).await?;

    axum::serve(listener, app).await?;
    Ok(state.await?)
}
