mod state;
mod models;
mod routes;

use tokio::sync::mpsc;
use actix_web::{web, App, HttpServer};

const DEFAULT_MESSAGE_CAPACITY: usize = 32;
static LOCAL_HOST: &str = "127.0.0.1";

pub async fn start_web_server(port_number: u16) {
    log::debug!("Starting the web server on {} port....", port_number);

    let (tx, rx) = mpsc::channel(DEFAULT_MESSAGE_CAPACITY);

    let mut state = state::State::new(rx);

    let state = tokio::spawn(async move {
        state.run().await;
    });

    let _ = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(tx.clone()))
            .service(routes::list_todos)
            .service(routes::create_todo)
            .service(routes::delete_todo)
            .service(routes::update_todo)
    })
    .bind((LOCAL_HOST, port_number)).unwrap()
    .run().await;

    state.await.unwrap();
}