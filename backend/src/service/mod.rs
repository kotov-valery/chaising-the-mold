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

const GET_MEASUREMENTS_URI: &str = "/measurements";

pub struct HttpBackand {
    tx: mpsc::Sender<state::Message>,
    state: Option<state::State>,
}

impl HttpBackand {
    pub fn new(storage: Box<dyn Storage + Send>) -> Self {
        let (tx, rx) = mpsc::channel(DEFAULT_MESSAGE_CAPACITY);
        let state = state::State::new(rx, storage);
        Self { tx, state: Some(state) }
    }
    pub async fn start(
        mut self,
        host_addr: &str,
        port_number: u16,
    ) -> Self {
        let state = tokio::spawn(async move {
            self.state.unwrap().run().await;
        });
        self.state = None;

        let tx = self.tx.clone();
        let _ = HttpServer::new(move || {
            App::new().app_data(web::Data::new(tx.clone())).route(
                GET_MEASUREMENTS_URI,
                web::get().to(routes::list_measurements),
            )
        })
        .bind((host_addr, port_number))
        .unwrap()
        .run()
        .await;

        state.await.unwrap();
        self
    }

    pub async fn stop(&self) {
        let _ = self.tx.send(state::Message::Stop).await;
    }
}

#[cfg(test)]
mod integration_test {
    use crate::sensing::sensor::DataPoint;
    use crate::storage::MockStorage;

    use super::*;
    use actix_web::{body, test, web, App};

    #[actix_web::test]
    async fn fetch_dummy_data() {
        let (tx, rx) = mpsc::channel(DEFAULT_MESSAGE_CAPACITY);

        let mut storage = MockStorage::new();
        let storage_data = vec![DataPoint {
            temperature: 20.0,
            humidity: 45.5,
        }];
        storage.expect_read().return_const(storage_data.clone());

        let mut state = state::State::new(rx, Box::new(storage));
        let state = tokio::spawn(async move {
            state.run().await;
        });

        let app = test::init_service(App::new().app_data(web::Data::new(tx.clone())).route(
            GET_MEASUREMENTS_URI,
            web::get().to(routes::list_measurements),
        ))
        .await;
        let req = test::TestRequest::get()
            .uri(GET_MEASUREMENTS_URI)
            .to_request();

        let resp = test::call_service(&app, req).await;

        assert!(resp.status().is_success());

        let body = resp.into_body();
        let bytes = body::to_bytes(body).await;
        assert_eq!(
            bytes.unwrap(),
            web::Bytes::from_static(b"[{\"temperature\":\"20.00\",\"humidity\":\"45.50\"}]")
        );

        let _ = tx.send(state::Message::Stop).await;
        state.await.unwrap();
    }
}
