///
/// Module is responsible to serve REST API to read the data
///
mod models;
mod routes;
mod state;

use actix_web::{web, App, HttpServer};
use tokio::sync::mpsc;

use crate::storage::Storage;

const DEFAULT_MESSAGE_CAPACITY: usize = 32;

pub struct HttpBackand {}

impl HttpBackand {
    pub async fn start(host_addr: &str, port_number: u16, storage: Box<dyn Storage + Send>) {
        let (tx, rx) = mpsc::channel(DEFAULT_MESSAGE_CAPACITY);

        let mut state = state::State::new(rx, storage);

        let state = tokio::spawn(async move {
            state.run().await;
        });

        let _ = HttpServer::new(move || {
            App::new()
                .app_data(web::Data::new(tx.clone()))
                .service(routes::list_measurements)
        })
        .bind((host_addr, port_number))
        .unwrap()
        .run()
        .await;

        state.await.unwrap();
    }
}
