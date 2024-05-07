pub mod config;
pub mod discord;
pub mod parser;
pub mod processes;
pub mod spotify;

pub mod prelude {
    pub use crate::config::*;
    pub use tracing::{debug, error, info, instrument, trace, warn};
}

use clap::Parser;
use discord::update_activity;
use parser::{cli::parse_command, Cli};
use prelude::*;
use std::process::ExitCode;
use tracing::Level;
// use tracing_appender::rolling;
use tracing_subscriber::fmt;

#[tokio::main]
async fn main() -> ExitCode {
    let args: Cli = Cli::parse();
    log_setup(args.debug, args.verbose);

    let mut config: Config = match initialize_config(args.config_overwrite) {
        Err(_) => return ExitCode::FAILURE,
        Ok(config) => config,
    };
    trace!("Config:\n{config:#?}");

    return match parse_command(&mut config, args).await {
        Err(_) => ExitCode::FAILURE,
        Ok(result) => {
            if let Some(mut client) = result {
                loop {
                    std::thread::sleep(std::time::Duration::from_secs(3));
                    client = match update_activity(&mut config, client).await {
                        Err(_) => return ExitCode::FAILURE,

                        Ok(client) => client,
                    }
                }
            }
            ExitCode::SUCCESS
        }
    };
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

        debug!("Debug level");
        trace!("Trace level");
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
