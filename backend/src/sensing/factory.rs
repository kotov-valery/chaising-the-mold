use super::config::Config;
use super::sensor::Sensor;
use super::uart::UartSensor;

#[cfg_attr(test, mockall::automock)]
pub trait SensorFactory {
    fn create(&self, config: Config) -> Box<dyn Sensor>;
}

pub struct DefaultSensorFactory {}

impl SensorFactory for DefaultSensorFactory {
    fn create(&self, config: Config) -> Box<dyn Sensor> {
        match config {
            Config::Uart(device, baud) => Box::new(UartSensor::new(device, baud)),
        }
    }
}
