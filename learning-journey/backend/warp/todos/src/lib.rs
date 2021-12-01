mod models;
mod filters;
mod handlers;

use filters::API;
use models::create_empty_storage;

const LOCAL_HOST: [u8; 4] = [127,0,0,1];

pub async fn start_web_server(port_number: u16) {
    log::debug!("Starting the web server on {} port....", port_number);

    let storage = create_empty_storage();
    let api = API::new(&storage);

    warp::serve(api.get_routes()).run((LOCAL_HOST, port_number)).await;
}