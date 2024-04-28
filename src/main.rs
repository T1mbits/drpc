pub mod config;
pub mod discord;
pub mod parser;

use clap::Parser;
use config::{
    // dir_path,
    initialize_config,
    Config,
};
use parser::{cli::parse_command, Cli};
use tracing::{debug, Level};
// use tracing_appender::rolling;
use tracing_subscriber::fmt;

fn main() -> () {
    let args: Cli = Cli::parse();
    log_setup(args.debug);

    let config: Config = initialize_config();

    parse_command(config, args);

    loop {
        std::thread::sleep(std::time::Duration::from_secs(10));
    }
}

pub fn log_setup(debug: bool) -> () {
    if debug {
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
        return;
    }
    // let log_files = rolling::never(dir_path() + "logs/", "ddrpc.log");

    fmt()
        // .with_writer(log_files)
        // .with_ansi(false)
        .with_target(false)
        .with_max_level(Level::INFO)
        .init();
}
