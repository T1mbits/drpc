pub mod config;
pub mod ipc;
pub mod spotify;

pub mod log {
    pub use log::{debug, error, info, trace, warn};
    pub use simplelog::LevelFilter;
    use simplelog::*;

    /// Initialize global program logger. Will panic if run more than once
    pub fn log_init(log_level: LevelFilter) -> () {
        TermLogger::init(
            log_level,
            ConfigBuilder::new()
                .set_level_color(Level::Trace, Some(Color::Cyan))
                .set_level_color(Level::Debug, Some(Color::Blue))
                .set_level_color(Level::Info, Some(Color::Green))
                .add_filter_allow_str("ddrpc")
                .add_filter_allow_str("common")
                .build(),
            TerminalMode::Mixed,
            ColorChoice::Auto,
        )
        .expect("logger should not be set more than once");
    }
}

use std::fmt::Display;

/// Display an Option\<T> as either its display value or \<None>.
pub fn as_string<T>(value: &Option<T>) -> String
where
    T: Display,
{
    match value {
        Some(val) => val.to_string(),
        None => "<None>".to_string(),
    }
}
