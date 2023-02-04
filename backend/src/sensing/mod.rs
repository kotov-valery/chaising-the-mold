pub mod config;
pub mod factory;
///
/// Module represents working with various sensors
///
pub mod sensor;

mod uart;

#[cfg(test)]
mod tests {
    use float_cmp::approx_eq;

    use super::config::Config;
    use super::factory::{MockSensorFactory, SensorFactory};
    use super::sensor::{DataPoint, MockSensor};

    #[test]
    fn read_sensor_data() {
        let sensor_data = DataPoint {
            temperature: 20.5,
            humidity: 46.6,
        };

        let mut factory_mock = MockSensorFactory::new();
        factory_mock.expect_create().returning(move |_| {
            let mut sensor_mock = MockSensor::new();
            sensor_mock
                .expect_read_data()
                .return_const(Some(sensor_data));
            Box::new(sensor_mock)
        });
        let factory: &dyn SensorFactory = &factory_mock;

        let mut sensor = factory.create(Config::Uart(String::from("dummy location"), 9600));
        let read_data = sensor.read_data();
        assert!(approx_eq!(
            f32,
            sensor_data.temperature,
            read_data.unwrap().temperature,
            ulps = 2
        ));
        assert!(approx_eq!(
            f32,
            sensor_data.humidity,
            read_data.unwrap().humidity,
            ulps = 2
        ));
    }
}
