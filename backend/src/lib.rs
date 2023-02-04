use std::sync::{Arc, Mutex};
use std::thread;

use sensing::config;
use sensing::factory;

use service::HttpBackand;
use storage::circular_buffer::CircularBuffer;

use crate::sensing::factory::SensorFactory;
use crate::sensing::sensor;
use crate::storage::Storage;

pub mod sensing;
pub mod service;
pub mod storage;

const DEFAULT_STORAGE_CAPACITY: usize = 128;

pub fn add_dummy_test_data(storage: &mut dyn Storage) {
    storage.write(sensor::DataPoint {
        temperature: 20.0,
        humidity: 45.5,
    });
    storage.write(sensor::DataPoint {
        temperature: 10.0,
        humidity: 15.5,
    });
    storage.write(sensor::DataPoint {
        temperature: 22.5,
        humidity: 46.6,
    });
    storage.write(sensor::DataPoint {
        temperature: 19.3,
        humidity: 38.9,
    });
    storage.write(sensor::DataPoint {
        temperature: 22.5,
        humidity: 46.6,
    });
    storage.write(sensor::DataPoint {
        temperature: 19.3,
        humidity: 38.9,
    });
}

pub async fn start_web_service(
    host_addr: &str,
    port_number: u16,
    config: config::Config,
    add_dummy_data: bool,
) {
    log::info!(
        "Starting the web server on {} address on {} port....",
        host_addr,
        port_number
    );

    let mut storage = CircularBuffer::new(DEFAULT_STORAGE_CAPACITY);
    if add_dummy_data {
        add_dummy_test_data(&mut storage);
    }

    let storage = Arc::new(Mutex::new(storage));

    let storage_writer = Arc::clone(&storage);
    let sensor = thread::spawn(move || {
        let sensor_factory = factory::DefaultSensorFactory {};
        let mut sensor = sensor_factory.create(config);
        loop {
            if let Some(data) = sensor.read_data() {
                println!("Read data from serial: {:?}", data);
                let mut storage = storage_writer.lock().unwrap();
                storage.write(data);
            } else {
                println!("Failed to read data from serial device");
            }
        }
    });

    let backend = HttpBackand::new(storage);
    backend.start(host_addr, port_number).await;

    sensor.join().unwrap();
}
