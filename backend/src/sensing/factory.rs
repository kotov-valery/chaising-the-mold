use super::config::Config;
use super::sensor::Sensor;

#[cfg_attr(test, mockall::automock)]
pub trait SensorFactory {
    fn create(&self, config: Config) -> Box<dyn Sensor>;
}
