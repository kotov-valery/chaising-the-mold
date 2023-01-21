///
/// Module is responsible to serve REST API to read the data
///
use crate::storage::Storage;

pub struct HttpBackand {
    storage: Box<dyn Storage>,
}

impl HttpBackand {
    pub fn new(storage: Box<dyn Storage>) -> Self {
        Self { storage }
    }

    pub fn start(&self, host_addr: &str, port_number: u16) {}
}
