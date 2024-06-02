mod models;
mod routes;
mod state;

use std::error::Error;
#[macro_use]
extern crate log;

use axum::{routing::get, Router};

use std::net::{Ipv4Addr, SocketAddrV4};
use std::sync::Arc;

pub async fn start_web_server(host_addr: &str, port_number: u16) -> Result<(), Box<dyn Error>> {
    info!(
        "Starting a web service on {} address on {} port...",
        host_addr, port_number
    );

    let state = Arc::new(state::AppState::new());
    let app = Router::new()
        .route("/todos", get(routes::list_todos))
        .with_state(state);

    let addr: Ipv4Addr = host_addr.parse()?;
    let listener = tokio::net::TcpListener::bind(SocketAddrV4::new(addr, port_number)).await?;
    Ok(axum::serve(listener, app).await?)
}
