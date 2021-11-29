/// Based on https://gitlab.com/susurrus/serialport-rs/-/blob/master/examples/receive_data.rs example
use std::io::{self, Write};
use std::time::Duration;

use clap::{App, Arg, AppSettings};

const SERIAL_TIMEOUT: Duration = Duration::from_millis(10);

fn main() {
    let matches = App::new("Read serial data")
        .setting(AppSettings::DisableVersion)
        .author("Valery Kotov <kotov.valery@gmail.com>")
        .about(concat!(
            "Application reads the data from the specified serial port ",
            "and echoes it to standard output"))
        .arg(
            Arg::with_name("device")
                .short("d")
                .long("device")
                .help("Path to a serial device")
                .required(true)
                .value_name("DEVICE")
                .use_delimiter(false)
                .takes_value(true)
        )
        .arg(
            Arg::with_name("baud")
                .short("b")
                .long("baud")
                .help("Baud rate for serial communication")
                .required(true)
                .value_name("BAUD_RATE")
                .use_delimiter(false)
                .takes_value(true)
        ).get_matches();

    let device = matches.value_of("device").unwrap_or("/dev/ttyUSB0");
    let baud = matches.value_of("baud").unwrap_or("115200").parse::<u32>().unwrap();

    let mut serial = serialport::new(device, baud)
        .timeout(SERIAL_TIMEOUT)
        .open()
        .expect(&format!("Failed to open {} device with {} baud rate", &device, &baud));

    let mut buffer: Vec<u8> = vec![0; 1000];
    println!("Receiving data on {} at {} baud:", &device, &baud);
    loop {
        match serial.read(buffer.as_mut_slice()) {
            Ok(t) => io::stdout().write_all(&buffer[..t]).unwrap(),
            Err(ref e) if e.kind() == io::ErrorKind::TimedOut => (),
            Err(e) => eprintln!("{:?}", e),
        }
    }
}
