use crate::storage::{DataPoint, Storage};
use std::collections::VecDeque;

pub struct CircularBuffer {
    capacity: usize,
    current_count: usize,
    buffer: VecDeque<DataPoint>,
}

impl CircularBuffer {
    pub fn new(capacity: usize) -> Self {
        Self {
            capacity: capacity,
            current_count: 0,
            buffer: VecDeque::new(),
        }
    }
}

impl Storage for CircularBuffer {
    fn read(&self) -> Vec<DataPoint> {
        Vec::from(self.buffer.clone())
    }

    fn write(&mut self, data: DataPoint) {
        if self.capacity > 0 {
            if self.current_count < self.capacity {
                self.buffer.push_back(data);
                self.current_count += 1;
            } else {
                self.buffer.pop_front();
                self.buffer.push_back(data);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::storage::Storage;
    use float_cmp::approx_eq;

    use super::{CircularBuffer, DataPoint};

    #[test]
    fn create_with_null_capacity() {
        let expected_data = DataPoint {
            temperature: 20.5,
            humidity: 46.6,
        };
        let mut buffer = CircularBuffer::new(0);
        buffer.write(expected_data);
        let read_data = buffer.read();

        assert_eq!(0, read_data.len());
    }

    parameterized_test::create! { create_and_simple_check, input_data, {
        let mut buffer = CircularBuffer::new(input_data.len());
        for entry in &input_data {
            buffer.write(entry.clone());
        }
        let read_data = buffer.read();

        assert_eq!(input_data.len(), read_data.len());
        for i in 0..read_data.len() {
            assert!( approx_eq!(f32, input_data[i].temperature, read_data[i].temperature, ulps = 2) );
            assert!( approx_eq!(f32, input_data[i].humidity, read_data[i].humidity, ulps = 2) );
        }
    }}
    create_and_simple_check! {
        single_entry: vec![DataPoint {temperature: 20.5, humidity: 46.6, }],
        multiple_entries: vec![
            DataPoint {temperature: 20.5, humidity: 46.6, },
            DataPoint {temperature: 25.5, humidity: 40.3, }
        ],
    }

    #[test]
    fn rewrite_the_data() {
        let mut buffer = CircularBuffer::new(1);
        buffer.write(DataPoint {
            temperature: 10.0,
            humidity: 15.5,
        });
        let expected_data = DataPoint {
            temperature: 20.5,
            humidity: 46.6,
        };
        buffer.write(expected_data);
        let read_data = buffer.read();

        assert_eq!(1, read_data.len());
        assert!(approx_eq!(
            f32,
            expected_data.temperature,
            read_data[0].temperature,
            ulps = 2
        ));
        assert!(approx_eq!(
            f32,
            expected_data.humidity,
            read_data[0].humidity,
            ulps = 2
        ));
    }

    parameterized_test::create! { rewrite_the_data_multiple_items, (capacity, input_data, expected_data), {
        let mut buffer = CircularBuffer::new(capacity);
        for data in &input_data {
            buffer.write(data.clone());
        }
        let read_data = buffer.read();

        assert_eq!(expected_data.len(), read_data.len());
        for i in 0..read_data.len() {
            assert!( approx_eq!(f32, expected_data[i].temperature, read_data[i].temperature, ulps = 2) );
            assert!( approx_eq!(f32, expected_data[i].humidity, read_data[i].humidity, ulps = 2) );
        }
    }}
    rewrite_the_data_multiple_items! {
        one_replacement: (
            2,
            vec![
                DataPoint{ temperature: 20.0, humidity: 45.5 },
                DataPoint{ temperature: 10.0, humidity: 15.5 },
                DataPoint{ temperature: 22.5, humidity: 46.6 },
            ],
            vec![
                DataPoint{ temperature: 10.0, humidity: 15.5 },
                DataPoint{ temperature: 22.5, humidity: 46.6 },
            ],
        ),
        set_is_replaced: (
            2,
            vec![
                DataPoint{ temperature: 20.0, humidity: 45.5 },
                DataPoint{ temperature: 10.0, humidity: 15.5 },
                DataPoint{ temperature: 22.5, humidity: 46.6 },
                DataPoint{ temperature: 19.3, humidity: 38.9 },
            ],
            vec![
                DataPoint{ temperature: 22.5, humidity: 46.6 },
                DataPoint{ temperature: 19.3, humidity: 38.9 },
            ],
        ),
    }
}
