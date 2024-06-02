/// Provides a RESTful web server managing some Todos.
///
/// API will be:
///
/// - `GET /todos`: return a JSON list of Todos.
/// - `POST /todos`: create a new Todo.
/// - `PUT /todos/:id`: update a specific Todo.
/// - `DELETE /todos/:id`: delete a specific Todo.
extern crate pretty_env_logger;

use clap::Parser;
use todos;

/// Provides a RESTful web server managing TODOs list.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// IP address to host service on
    #[arg(short = 'a', long)]
    host_addr: String,
    /// Port number to host service on
    #[arg(short, long)]
    port_number: u16,
}

fn main() {
    pretty_env_logger::init();
    let args = Args::parse();
    todos::start_web_server(&args.host_addr, args.port_number);
}
