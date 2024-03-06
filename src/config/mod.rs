pub mod structure;

use dirs::config_dir;
use std::{fs, path::Path};
use toml::{from_str, to_string};

use structure::DConfig;

/// Generates the directory for where the config file should be located.
///
/// Use this function in case of different OS or no accessible configuration directory environment variable
fn generate_config_dir_path() -> String {
    match config_dir() {
        Some(config_dir) => match config_dir.to_str() {
            None => "./ddrpc".to_owned(),
            Some(config_dir) => config_dir.to_owned() + "/ddrpc",
        },
        None => "./ddrpc".to_owned(),
    }
}

/// Generates the file location for where the config file should be located.
///
/// Uses `generate_config_dir_path()` and appends file location.
fn generate_config_file_path() -> String {
    generate_config_dir_path() + "/ddrpc.toml"
}

pub fn initialize_config() -> DConfig {
    if Path::new(&generate_config_file_path()).exists() {
        return read_config_file();
    };
    DConfig::default()
}

pub fn write_config(config: &DConfig) -> () {
    let config_dir: String = generate_config_dir_path();
    let config_file: String = generate_config_file_path();

    let serialized_config: String = match to_string(config) {
        Ok(serialized_config) => serialized_config,
        Err(error) => panic!("Error while serializing config data: {}", error),
    };

    if !Path::new(&config_dir).exists() {
        match fs::create_dir_all(&config_dir) {
            Err(error) => panic!("Error while creating config directory: {}", error),
            Ok(_) => println!("Created directory {}", config_dir),
        }
    }

    match fs::write(&config_file, serialized_config) {
        Ok(_) => println!("Wrote to file {}", config_file),
        Err(error) => panic!("Error while writing config: {}", error),
    }
}

pub fn read_config_file() -> DConfig {
    let config_file: String = generate_config_file_path();
    match fs::read(&config_file) {
        Ok(config_vector) => verify_config_integrity(config_vector, config_file),
        Err(error) => panic!("Error while reading config at {}: {}", config_file, error),
    }
}

fn verify_config_integrity(config_vector: Vec<u8>, config_file: String) -> DConfig {
    let config_string: String = match String::from_utf8(config_vector) {
        Err(_) => panic!("There's no way that's a valid config file"),
        Ok(decoded_string) => decoded_string,
    };
    match from_str(&config_string) {
        Err(error) => {
            println!("Error while deserializing configuration file: {}", error);
            match fs::remove_file(config_file) {
                Ok(_) => println!("Removed invalid configuration file"),
                Err(error) => panic!("Error while removing invalid configuration file: {}", error),
            }
            initialize_config()
        }
        Ok(config) => config,
    }
}
