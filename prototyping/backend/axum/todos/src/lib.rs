#[macro_use]
extern crate log;

pub fn start_web_server(host_addr: &str, port_number: u16) {
    info!(
        "Starting a web service on {} address on {} port...",
        host_addr, port_number
    );
}
