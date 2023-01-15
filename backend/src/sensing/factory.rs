use super::sensor::Sensor;
use super::config::Config;

#[cfg_attr(test, mockall::automock)]
pub trait SensorFactory {
    fn create(&self, config: Config) -> Box<dyn Sensor>;
}