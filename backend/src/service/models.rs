use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct Measurement {
    pub temperature: String,
    pub humidity: String,
}

