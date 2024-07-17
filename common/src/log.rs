pub use log::{debug, error, info, trace, warn};
pub use simplelog::LevelFilter;
use simplelog::*;

/// Initialize global program logger. Will panic if run more than once
pub fn log_init(log_level: LevelFilter) {
    TermLogger::init(
        log_level,
        ConfigBuilder::new()
            .set_level_color(Level::Trace, Some(Color::Cyan))
            .set_level_color(Level::Debug, Some(Color::Blue))
            .set_level_color(Level::Info, Some(Color::Green))
            .build(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    )
    .expect("logger should not be set more than once");
}
