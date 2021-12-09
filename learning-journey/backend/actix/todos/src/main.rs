/// Provides a RESTful web server managing some Todos.
///
/// API will be:
///
/// - `GET /todos`: return a JSON list of Todos.
/// - `POST /todos`: create a new Todo.
/// - `PUT /todos/:id`: update a specific Todo.
/// - `DELETE /todos/:id`: delete a specific Todo.

use std::env;
use clap::{App, Arg, AppSettings};

use todos::start_web_server;

static LOCAL_HOST: &str = "127.0.0.1";

#[actix_web::main]
async fn main() {
    let matches = App::new("TODO RESTful server")
        .author("Valery Kotov <kotov.valery@gmail.com>")
        .setting(AppSettings::DisableVersion)
        .about("")
        .arg(
            Arg::with_name("port")
            .short("p")
            .long("port")
            .help("Specify port number to start the service")
            .value_name("PORT")
            .default_value("3030")
            .takes_value(true)
            .use_delimiter(false)
            .required(true)
        )
        .arg(
            Arg::with_name("host")
            .short("a")
            .long("host")
            .help("Specify ip address to run the server")
            .default_value(LOCAL_HOST)
            .takes_value(true)
            .use_delimiter(false)
            .required(true)
        )
        .arg(
            Arg::with_name("verbose")
            .short("v")
            .long("verbose")
            .help("Enable verbose logging")
            .use_delimiter(false)
            .takes_value(false)
            .required(false)
        )
        .get_matches();

    let enable_verbose_logging = matches.is_present("verbose");
    let port_number = matches.value_of("port").unwrap().parse::<u16>().unwrap();
    let host_addr = matches.value_of("host").unwrap();

    // Set `RUST_LOG=todos=debug` or pass `-v` or `--verbose`
    // as command-line argument to see debug logs.
    if env::var_os("RUST_LOG").is_none() {
        env::set_var("RUST_LOG", "actix_web=info,actix_server=info");
    }
    if enable_verbose_logging {
        env::set_var("RUST_LOG", "actix_web=debug,actix_server=debug");
    }
    pretty_env_logger::init();

    start_web_server(host_addr, port_number).await;
}
