pub mod config;
pub mod daemon;
pub mod discord;
pub mod logging;
pub mod parser;

use config::{initialize_config, DConfig};
use parser::cli::parse_command;

fn main() {
    let config: DConfig = initialize_config();
    parse_command(config);
}
