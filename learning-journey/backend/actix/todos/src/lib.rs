mod state;
mod models;
mod routes;

use tokio::sync::mpsc;
use actix_web::{web, App, HttpServer};

const DEFAULT_MESSAGE_CAPACITY: usize = 32;

pub async fn start_web_server(port_number: u16) {
    log::debug!("Starting the web server on {} port....", port_number);

    let (tx, rx) = mpsc::channel(DEFAULT_MESSAGE_CAPACITY);

    let mut state = state::State::new(rx);

    //let state = tokio::spawn(async move {
    //    state.run().await;
    //});

    let _ = HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(routes::greet))
            .route("/{name}", web::get().to(routes::greet))
    })
    .bind(("127.0.0.1", 8080)).unwrap()
    .run().await;

    //state.await.unwrap();
}