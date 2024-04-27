pub mod config;
pub mod discord;
pub mod parser;

use config::{
    // dir_path,
    initialize_config,
    DConfig,
};
use parser::cli::parse_command;
use tracing::{debug, Level};
// use tracing_appender::rolling;
use tracing_subscriber::fmt;

fn main() {
    let config: DConfig = initialize_config();
    parse_command(config);
    loop {
        std::thread::sleep(std::time::Duration::from_secs(10));
    }
}

pub fn log_setup() -> () {
    // let log_files = rolling::never(dir_path() + "logs/", "ddrpc.log");

    fmt()
        // .with_writer(log_files)
        // .with_ansi(false)
        .with_target(false)
        .with_max_level(Level::INFO)
        .init();
}

pub fn log_setup_debug() -> () {
    // let log_files = rolling::never(dir_path() + "logs/", "ddrpc_debug.log");

    // let filter = EnvFilter::try_from_default_env()
    //     .or_else(|_| EnvFilter::try_new("warn"))
    //     .unwrap()
    //     .add_directive("ddrpc=trace".parse().unwrap());

    fmt()
        // .with_writer(log_files)
        // .with_ansi(false)
        // .with_env_filter(filter)
        .with_max_level(Level::TRACE)
        .init();

    debug!("Logger setup");
}
