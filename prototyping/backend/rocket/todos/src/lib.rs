mod state;
mod models;
mod routes;

#[macro_use] extern crate rocket;

use rocket::routes;
use tokio::sync::mpsc;

const DEFAULT_MESSAGE_CAPACITY: usize = 32;

pub async fn start_web_server(host_addr: &str, port_number: u16) {
    log::debug!("Starting the web server on {} port....", port_number);

    let figment = rocket::Config::figment()
        .merge(("address", host_addr))
        .merge(("port", port_number));

    let (tx, rx) = mpsc::channel(DEFAULT_MESSAGE_CAPACITY);

    let mut state = state::State::new(rx);

    let state = tokio::spawn(async move {
        state.run().await;
    });

    let _ = rocket::custom(figment)
        .manage(tx.clone())
        .mount("/", routes![
            routes::list_todos,
            routes::create_todo,
            routes::delete_todo,
            routes::update_todo,
        ])
        .launch().await;

    state.await.unwrap();
}
