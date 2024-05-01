pub mod config;
pub mod discord;
pub mod parser;
pub mod processes;

use clap::Parser;
use config::{
    // dir_path,
    initialize_config,
    Config,
};
use discord::update_activity;
use parser::{cli::parse_command, Cli};
use tracing::{debug, trace, Level};
// use tracing_appender::rolling;
use tracing_subscriber::fmt;

fn main() -> () {
    let args: Cli = Cli::parse();
    log_setup(args.debug, args.verbose);

    let mut config: Config = initialize_config(args.config_overwrite);

    if let Some(mut client) = parse_command(&mut config, args) {
        loop {
            std::thread::sleep(std::time::Duration::from_secs(3));
            client = update_activity(&mut config, client);
        }
    }

    return ();
}

/// Initializes logging subscriber defaults. If `debug` is true, log level will be `debug`. If `verbose` is true, log level will be `trace`. Otherwise, log level will be `info`.
#[tracing::instrument(skip_all)]
pub fn log_setup(debug: bool, verbose: bool) -> () {
    if debug || verbose {
        // let log_files = rolling::never(dir_path() + "logs/", "ddrpc_debug.log");

        // let filter = EnvFilter::try_from_default_env()
        //     .or_else(|_| EnvFilter::try_new("warn"))
        //     .unwrap()
        //     .add_directive("ddrpc=trace".parse().unwrap());

        fmt()
            // .with_writer(log_files)
            // .with_ansi(false)
            // .with_env_filter(filter)
            .with_max_level(if verbose { Level::TRACE } else { Level::DEBUG })
            .init();

        debug!("Debug logger initialized");
        trace!("Trace logger initialized");
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
