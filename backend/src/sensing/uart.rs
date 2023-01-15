use super::sensor::{DataPoint, Sensor};

pub struct UartSensor {
    location: String,
    baudrate: usize
}

impl UartSensor {
    pub fn new(location: String, baudrate: usize) -> Self {
        Self {location, baudrate}
    }
}

impl Sensor for UartSensor {
    fn read_data(&self) -> Option<DataPoint> {
        Some( DataPoint { temperature: 0.0, humidity: 0.0 } )
    }
}