use sensing::sensor::DataPoint;
use service::HttpBackand;
use storage::{circular_buffer::CircularBuffer, Storage};

pub mod sensing;
pub mod service;
pub mod storage;

const DEFAULT_STORAGE_CAPACITY: usize = 128;

fn add_dummy_test_data(storage: &mut dyn Storage) {
    storage.write(DataPoint {
        temperature: 20.0,
        humidity: 45.5,
    });
    storage.write(DataPoint {
        temperature: 10.0,
        humidity: 15.5,
    });
    storage.write(DataPoint {
        temperature: 22.5,
        humidity: 46.6,
    });
    storage.write(DataPoint {
        temperature: 19.3,
        humidity: 38.9,
    });
    storage.write(DataPoint {
        temperature: 22.5,
        humidity: 46.6,
    });
    storage.write(DataPoint {
        temperature: 19.3,
        humidity: 38.9,
    });
}

pub async fn start_web_service(host_addr: &str, port_number: u16) {
    log::info!(
        "Starting the web server on {} address on {} port....",
        host_addr,
        port_number
    );

    let mut storage = CircularBuffer::new(DEFAULT_STORAGE_CAPACITY);
    add_dummy_test_data(&mut storage);

    let backend = HttpBackand::new(Box::new(storage));
    backend
        .start(host_addr, port_number)
        .await;
}
