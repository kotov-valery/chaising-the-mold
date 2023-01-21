#[derive(Debug, Clone, Copy)]
pub struct DataPoint {
    pub temperature: f32,
    pub humidity: f32,
}

#[cfg_attr(test, mockall::automock)]
pub trait Sensor {
    fn read_data(&self) -> Option<DataPoint>;
}
