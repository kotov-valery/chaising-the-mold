mod models;
mod filters;
mod handlers;
mod state;

use filters::API;
use tokio::sync::mpsc;

const LOCAL_HOST: [u8; 4] = [127,0,0,1];
const DEFAULT_MESSAGE_CAPACITY: usize = 32;

pub async fn start_web_server(port_number: u16) {
    log::debug!("Starting the web server on {} port....", port_number);

    let (tx, rx) = mpsc::channel(DEFAULT_MESSAGE_CAPACITY);

    let mut state = state::State::new(rx);
    let api = API::new(tx.clone());

    let state = tokio::spawn(async move {
        state.run().await;
    });
    warp::serve(api.get_routes()).run((LOCAL_HOST, port_number)).await;
    state.await.unwrap();
}