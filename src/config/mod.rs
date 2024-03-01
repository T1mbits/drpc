pub mod structure;

use std::{fs, path::Path};
use toml::{from_str, to_string};

use structure::DConfig;

const CONFIG_PATH: &str = "../../drpc.toml";

pub fn initialize_config() -> DConfig {
    if Path::new(CONFIG_PATH).exists() {
        return read_config_file();
    };
    DConfig::default()
}

pub fn write_config(config: &DConfig) -> () {
    let serialized_config: String = match to_string(config) {
        Ok(serialized_config) => serialized_config,
        Err(error) => panic!("Error while serializing config data: {}", error),
    };
    match fs::write(CONFIG_PATH, serialized_config) {
        Ok(_) => {}
        Err(error) => panic!("Error while writing config: {}", error),
    }
}

pub fn read_config_file() -> DConfig {
    match fs::read(CONFIG_PATH) {
        Ok(config) => {
            match from_str(
                String::from_utf8(config)
                    .expect("umm what did you do to my config?")
                    .as_str(),
            ) {
                Ok(config) => config,
                Err(error) => panic!("Error while deserializing config: {}", error),
            }
        }
        Err(error) => panic!("Error while reading config at {}: {}", CONFIG_PATH, error),
    }
}
