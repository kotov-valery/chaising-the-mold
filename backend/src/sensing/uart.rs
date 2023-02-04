use std::io;
use std::time::Duration;

use serialport::SerialPort;

use super::sensor::{DataPoint, Sensor};

const SERIAL_TIMEOUT: Duration = Duration::from_millis(1000);

pub struct UartSensor {
    serial: Box<dyn SerialPort>,
}

impl UartSensor {
    pub fn new(location: String, baudrate: u32) -> Self {
        let serial = serialport::new(&location.clone(), baudrate)
                .timeout(SERIAL_TIMEOUT)
                .open()
                .expect(&format!("Failed to open {} device with {} baud rate", location, baudrate));
        Self {  serial }
    }
}

impl Sensor for UartSensor {
    fn read_data(&mut self) -> Option<DataPoint> {
        let mut string_data = String::new();
        loop {
            let mut buffer: Vec<u8> = vec![0; 1000];
            match self.serial.read(buffer.as_mut_slice()) {
                Ok(count) => {
                    let read_data = &buffer[..count];
                    let new_data = String::from_utf8(read_data.to_vec()).unwrap();
                    string_data.push_str(&new_data);
                    let string_data = string_data.trim_end();
                    if let Ok(data_point) = serde_json::from_str::<DataPoint>(&string_data) {
                        return Some(data_point);
                    }
            },
            Err(ref e) if e.kind() == io::ErrorKind::TimedOut => {
                log::debug!("Timeout to read data from serial");
                string_data.clear();
            },
            Err(e) => {
                log::error!("Failed to read data from serial: {:?}", e);
                return None;
            }
        }
        }
    }
}