/// Provides a RESTful web server managing some Todos.
///
/// API will be:
///
/// - `GET /todos`: return a JSON list of Todos.
/// - `POST /todos`: create a new Todo.
/// - `PUT /todos/:id`: update a specific Todo.
/// - `DELETE /todos/:id`: delete a specific Todo.
///
/// Base on https://github.com/seanmonstar/warp/blob/master/examples/todos.rs example

use std::env;
use clap::{arg, ArgAction, Command};

use todos::start_web_server;

#[tokio::main]
async fn main() {
    let matches = Command::new("todos")
        .author("Valery Kotov kotov.valery@gmail.com")
        .about("TODO RESTful server")
        .arg(arg!(-p --port <PORT> "Specify port number to start the service"))
        .arg(arg!(-a --host <HOST> "Specify ip address to run the server"))
        .arg(arg!(-v --verbose "Enable verbose logging").action(ArgAction::SetTrue))
        .get_matches();

    let enable_verbose_logging = *matches.get_one::<bool>("verbose").unwrap();
    let port_number = matches.get_one::<String>("port").unwrap().parse::<u16>().unwrap();
    let host_addr =  matches.get_one::<String>("host").unwrap();

    // Set `RUST_LOG=todos=debug` or pass `-v` or `--verbose`
    // as command-line argument to see debug logs.
    if env::var_os("RUST_LOG").is_none() {
        env::set_var("RUST_LOG", "todos=info");
    }
    if enable_verbose_logging {
        env::set_var("RUST_LOG", "todos=debug");
    }
    pretty_env_logger::init();

    start_web_server(host_addr, port_number).await;
}
