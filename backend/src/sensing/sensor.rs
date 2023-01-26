use serde::Deserialize;

#[derive(Debug, Clone, Copy, Deserialize)]
pub struct DataPoint {
    pub temperature: f32,
    pub humidity: f32,
}

#[cfg_attr(test, mockall::automock)]
pub trait Sensor {
    fn read_data(&mut self) -> Option<DataPoint>;
}
