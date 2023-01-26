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
        let mut buffer: Vec<u8> = vec![0; 2000];
        match self.serial.read(buffer.as_mut_slice()) {
            Ok(_) => {
                let string_data = String::from_utf8(buffer).unwrap();
                println!("Read data from serial: {}", string_data);
                let data_point = serde_json::from_str(&string_data);
                match data_point {
                    Ok(data) => Some(data),
                    _ => None,
                }
            },
            Err(ref e) if e.kind() == io::ErrorKind::TimedOut => None,
            Err(e) => {
                eprintln!("{:?}", e);
                None
            }
        }
    }
}