/// Based on https://gitlab.com/susurrus/serialport-rs/-/blob/master/examples/receive_data.rs example
use clap::{arg, Command};

use backend::sensing::config;
use backend::sensing::factory::{self, SensorFactory};

fn main() {
    pretty_env_logger::init();

    let matches = Command::new("Read serial data")
        .author("Valery Kotov <kotov.valery@gmail.com>")
        .about(concat!(
            "Application reads the data from the specified serial port ",
            "and echoes it to standard output"
        ))
        .arg(arg!(-d --device <DEVICE> "Specify path to a serial device"))
        .arg(arg!(-b --baud <BAUD_RATE> "Specify serial baud rate"))
        .get_matches();

    let device = matches.get_one::<String>("device").unwrap();
    let baud = matches.get_one::<String>("baud").unwrap();
    let baud = baud.parse::<u32>().unwrap();

    let sensor_factory = factory::DefaultSensorFactory {};
    let mut sensor = sensor_factory.create(config::Config::Uart(device.clone(), baud));
    loop {
        if let Some(data) = sensor.read_data() {
            println!("Read data from serial: {:?}", data);
        } else {
            println!("Failed to read data from serial device");
        }
    }
}
