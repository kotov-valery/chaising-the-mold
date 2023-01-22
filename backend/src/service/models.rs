use serde::Serialize;

use crate::sensing::sensor::DataPoint;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Measurement {
    pub temperature: String,
    pub humidity: String,
}

impl Measurement {
    pub fn from(data_point: &DataPoint) -> Self {
        Measurement {
            temperature: format!("{:.2}", data_point.temperature),
            humidity: format!("{:.2}", data_point.humidity),
        }
    }
}
