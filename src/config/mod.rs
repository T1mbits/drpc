mod structure;

pub use structure::*;

use dirs::config_dir;
use std::{fs, path::Path, process};
use toml::{from_str, to_string};
use tracing::{debug, error, instrument, trace, warn};

/// backslash included
pub fn dir_path() -> String {
    match config_dir() {
        Some(config_dir) => match config_dir.to_str() {
            None => "./ddrpc/".to_owned(),
            Some(config_dir) => config_dir.to_owned() + "/ddrpc/",
        },
        None => "./ddrpc/".to_owned(),
    }
}

fn file_path() -> String {
    dir_path() + "ddrpc.toml"
}

#[instrument]
pub fn initialize_config() -> Config {
    let file_path: &str = &file_path();
    debug!("Config file path: {file_path}");
    if Path::new(file_path).exists() {
        read_config_file()
    } else {
        warn!("Config file not found, creating new file with defaults");
        let default = Config::default();
        write_config(&default);
        default
    }
}

#[instrument(skip_all)]
pub fn write_config(config: &Config) -> () {
    let config_dir: String = dir_path();
    let config_file: String = file_path();

    let serialized_config: String = match to_string(config) {
        Ok(serialized_config) => {
            trace!("Serialized config");
            serialized_config
        }
        Err(error) => {
            error!("Error while serializing config data: {error}");
            process::exit(1);
        }
    };

    if !Path::new(&config_dir).exists() {
        match fs::create_dir_all(&config_dir) {
            Err(error) => {
                error!("Error while creating config directory: {error}");
                process::exit(1)
            }
            Ok(_) => trace!("Created config directory {config_dir}"),
        }
    }

    match fs::write(&config_file, serialized_config) {
        Ok(_) => debug!("Wrote to file {config_file}"),
        Err(error) => {
            error!("Error while writing config: {error}");
            process::exit(1);
        }
    }
}

#[instrument]
pub fn read_config_file() -> Config {
    let config_file: String = file_path();
    match fs::read(&config_file) {
        Ok(config_vector) => {
            debug!("Successfully read config file from {config_file}");
            verify_config_integrity(config_vector, config_file)
        }
        Err(error) => {
            error!("Error while reading config at {config_file}: {error}");
            process::exit(1);
        }
    }
}

#[instrument(skip_all)]
fn verify_config_integrity(config_vector: Vec<u8>, config_file: String) -> Config {
    let config_string: String = match String::from_utf8(config_vector) {
        Err(_) => {
            error!("There's no way that's a valid config file");
            process::exit(1)
        }
        Ok(decoded_string) => decoded_string,
    };
    match from_str(&config_string) {
        Err(error) => {
            warn!("Error while deserializing configuration file: {error}");
            match fs::remove_file(config_file) {
                Ok(_) => {
                    warn!("Removed invalid configuration file, creating new file");
                    write_config(&Config::default());
                    Config::default()
                }
                Err(error) => {
                    error!("Error while removing invalid configuration file: {error}");
                    process::exit(1);
                }
            }
        }
        Ok(config) => {
            trace!("Config file validated");
            config
        }
    }
}
