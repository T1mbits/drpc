pub mod structure;

use dirs::config_dir;
use std::{fs, path::Path, process};
use toml::{from_str, to_string};

use crate::config::structure::DConfig;

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

pub fn initialize_config() -> DConfig {
    if Path::new(&file_path()).exists() {
        read_config_file()
    } else {
        write_config(&DConfig::default());
        read_config_file()
    }
}

pub fn write_config(config: &DConfig) -> () {
    let config_dir: String = dir_path();
    let config_file: String = file_path();

    let serialized_config: String = match to_string(config) {
        Ok(serialized_config) => serialized_config,
        Err(error) => {
            eprintln!("Error while serializing config data: {}", error);
            process::exit(1);
        }
    };

    if !Path::new(&config_dir).exists() {
        match fs::create_dir_all(&config_dir) {
            Err(error) => {
                eprintln!("Error while creating config directory: {}", error);
                process::exit(1)
            }
            Ok(_) => println!("Created directory {}", config_dir),
        }
    }

    match fs::write(&config_file, serialized_config) {
        Ok(_) => println!("Wrote to file {}", config_file),
        Err(error) => {
            eprintln!("Error while writing config: {}", error);
            process::exit(1);
        }
    }
}

pub fn read_config_file() -> DConfig {
    let config_file: String = file_path();
    match fs::read(&config_file) {
        Ok(config_vector) => verify_config_integrity(config_vector, config_file),
        Err(error) => {
            eprintln!("Error while reading config at {}: {}", config_file, error);
            process::exit(1);
        }
    }
}

fn verify_config_integrity(config_vector: Vec<u8>, config_file: String) -> DConfig {
    let config_string: String = match String::from_utf8(config_vector) {
        Err(_) => {
            eprintln!("There's no way that's a valid config file");
            process::exit(1)
        }
        Ok(decoded_string) => decoded_string,
    };
    match from_str(&config_string) {
        Err(error) => {
            eprintln!("Error while deserializing configuration file: {}", error);
            match fs::remove_file(config_file) {
                Ok(_) => {
                    println!("Removed invalid configuration file");
                    write_config(&DConfig::default());
                    initialize_config()
                }
                Err(error) => {
                    eprintln!("Error while removing invalid configuration file: {}", error);
                    process::exit(1);
                }
            }
        }
        Ok(config) => config,
    }
}
