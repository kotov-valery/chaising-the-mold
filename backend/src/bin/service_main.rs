///
/// Provides a RESTful web server with telemetry
///
use backend::start_web_service;

use backend::sensing::config;

use clap::{arg, ArgAction, Command};
use std::env;

const DEFAULT_SERIAL_BAUD_RATE: u32 = 115200;

#[actix_web::main]
async fn main() {
    let matches = Command::new("chasing the mold")
        .author("Valery Kotov <kotov.valery@gmail.com>")
        .about("REST API to telemetery data")
        .arg(arg!(-p --port <PORT> "Specify port number to start the service"))
        .arg(arg!(-a --host <HOST> "Specify ip address to run the server"))
        .arg(arg!(-s --serial <SERIAL> "Read data from serial"))
        .arg(arg!(-d --dummy "Add dummy data for testing").action(ArgAction::SetFalse))
        .arg(arg!(-v --verbose "Enable verbose logging"))
        .get_matches();

    let port_number = matches
        .get_one::<String>("port")
        .unwrap()
        .parse::<u16>()
        .unwrap();
    let host_addr = matches.get_one::<String>("host").unwrap();
    let enable_verbose_logging = *matches.get_one::<bool>("verbose").unwrap();

    let device = matches.get_one::<String>("device");
    let add_dummy_data = *matches.get_one::<bool>("dummy").unwrap();

    // Set `RUST_LOG=todos=debug` or pass `-v` or `--verbose`
    // as command-line argument to see debug logs.
    if env::var_os("RUST_LOG").is_none() {
        env::set_var("RUST_LOG", "actix_web=info,actix_server=info");
    }
    if enable_verbose_logging {
        env::set_var("RUST_LOG", "actix_web=debug,actix_server=debug");
    }
    pretty_env_logger::init();

    if let Some(device) = device {
        start_web_service(
            host_addr,
            port_number,
            config::Config::Uart(device.clone(), DEFAULT_SERIAL_BAUD_RATE),
            add_dummy_data,
        )
        .await;
    }
}
