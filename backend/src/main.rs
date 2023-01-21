use backend::start_web_service;
///
/// Provides a RESTful web server with telemetry
///
use clap::{arg, Command};

fn main() {
    let matches = Command::new("chasing the mold")
        .author("Valery Kotov <kotov.valery@gmail.com>")
        .about("REST API to telemetery data")
        .arg(arg!(-p --port <PORT> "Specify port number to start the service"))
        .arg(arg!(-a --host <HOST> "Specify ip address to run the server"))
        .get_matches();

    let port_number = matches
        .get_one::<String>("port")
        .unwrap()
        .parse::<u16>()
        .unwrap();
    let host_addr = matches.get_one::<String>("host").unwrap();

    start_web_service(host_addr, port_number);
}
