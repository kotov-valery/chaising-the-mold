///
/// Module represents persistent storage
///
pub mod circular_buffer;

use crate::sensing::sensor::DataPoint;

#[cfg_attr(test, mockall::automock)]
pub trait Storage {
    fn read(&self) -> Vec<DataPoint>;
    fn write(&mut self, data: DataPoint);
}
