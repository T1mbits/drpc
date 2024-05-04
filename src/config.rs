use crate::{
    discord::{replace_template_variables, DiscordClientWrapper},
    prelude::*,
};
use dirs::config_dir;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs, path::Path};
use toml::{from_str, to_string};

/// Creates path to config directory specific to OS. A slash is appended to the end of the path
pub fn dir_path() -> String {
    return match config_dir() {
        Some(config_dir) => match config_dir.to_str() {
            None => "./ddrpc/".to_owned(),
            Some(config_dir) => config_dir.to_owned() + "/ddrpc/",
        },
        None => "./ddrpc/".to_owned(),
    };
}

/// Appends `ddrpc.toml` to the end of the path produced by `dir_path()`
fn file_path() -> String {
    return dir_path() + "ddrpc.toml";
}

/// Deserialize or create a new `Config` struct.
#[instrument(skip_all)]
pub fn initialize_config(overwrite: bool) -> Result<Config, ()> {
    let file_path: &str = &file_path();
    debug!("Config file path: {file_path}");
    if Path::new(file_path).exists() {
        trace!("Config file found");
        return match read_config_file(overwrite) {
            Err(_) => {
                error!("Invalid configuration file found. Use --config-overwrite to overwrite the invalid config file with default values.");
                Err(())
            }
            Ok(config) => Ok(config),
        };
    } else {
        warn!("Config file not found, creating new file with defaults");
        let default = Config::default();
        return match write_config(&default) {
            Err(_) => Err(()),
            Ok(_) => Ok(default),
        };
    }
}

/// Write config to the file at `file_path()`
#[instrument(skip_all)]
pub fn write_config(config: &Config) -> Result<Option<DiscordClientWrapper>, ()> {
    let config_dir: String = dir_path();
    let config_file: String = file_path();

    let serialized_config: String = match to_string(config) {
        Ok(serialized_config) => {
            trace!("Serialized config");
            serialized_config
        }
        Err(error) => {
            error!("Error while serializing config data: {error}");
            return Err(());
        }
    };

    if !Path::new(&config_dir).exists() {
        match fs::create_dir_all(&config_dir) {
            Err(error) => {
                error!("Error while creating config directory: {error}");
                return Err(());
            }
            Ok(_) => trace!("Created config directory {config_dir}"),
        }
    }

    return match fs::write(&config_file, serialized_config) {
        Ok(_) => {
            trace!("Wrote to file {config_file}");
            Ok(None)
        }
        Err(error) => {
            error!("Error while writing config: {error}");
            Err(())
        }
    };
}

/// Attempt to read and deserialize config from file. If an error occurs while deserializing, a default `Config` will be created.<br/>
/// If `overwrite` is set to false, an `Err(())` will be returned instead.
#[instrument(skip_all)]
pub fn read_config_file(overwrite: bool) -> Result<Config, ()> {
    let config_path: String = file_path();
    return match fs::read(&config_path) {
        Ok(config_vector) => {
            debug!("Successfully read config file from {config_path}");
            verify_config_integrity(config_vector, config_path, overwrite)
        }
        Err(error) => {
            error!("Error while reading config at {config_path}: {error}");
            Err(())
        }
    };
}

/// Deserialize string into `Config`. If an error occurs while deserializing, a default `Config` will be created.
/// If `overwrite` is set to false, an `Err(())` will be returned instead.
#[instrument(skip_all)]
fn verify_config_integrity(
    config_vector: Vec<u8>,
    config_path: String,
    overwrite: bool,
) -> Result<Config, ()> {
    let config_string: String = match String::from_utf8(config_vector) {
        Err(error) => {
            error!("Error while converting config to utf8: {error}");
            return Err(());
        }
        Ok(decoded_string) => {
            trace!("Successfully converted config file to utf8:\n{decoded_string}");
            decoded_string
        }
    };

    return match from_str(&config_string) {
        Err(error) => {
            warn!("Error while deserializing configuration file: {error}");
            trace!("Overwrite: {overwrite}");
            if !overwrite {
                return Err(());
            }
            match fs::remove_file(config_path) {
                Ok(_) => {
                    warn!("Removed invalid configuration file, creating new file with defaults");
                    match write_config(&Config::default()) {
                        Err(_) => Err(()),
                        Ok(_) => Ok(Config::default()),
                    }
                }
                Err(error) => {
                    error!("Error while removing invalid configuration file: {error}");
                    Err(())
                }
            }
        }
        Ok(config) => {
            trace!("Config file validated");
            Ok(config)
        }
    };
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Config {
    pub discord: DiscordConfig,
    pub processes: ProcessesConfig,
    pub spotify: SpotifyConfig,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            discord: DiscordConfig {
                assets: DiscordConfigAssets {
                    large_image: "".to_string(),
                    large_text: "".to_string(),
                    small_image: "".to_string(),
                    small_text: "".to_string(),
                },
                buttons: DiscordButtons {
                    btn1_text: "".to_string(),
                    btn1_url: "".to_string(),
                    btn2_text: "".to_string(),
                    btn2_url: "".to_string(),
                },
                client_id: 1133837522074607749,
                details: "".to_string(),
                state: "".to_string(),
            },
            processes: ProcessesConfig {
                idle_image: "idle".to_string(),
                idle_text: "Idle".to_string(),
                processes: vec![ProcessConfig {
                    image: "code".to_string(),
                    name: "code".to_string(),
                    text: "Visual Studio Code".to_string(),
                }],
            },
            spotify: SpotifyConfig {
                client_id: "".to_string(),
                client_secret: "".to_string(),
                refresh_token: "".to_string(),
            },
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct DiscordConfig {
    pub assets: DiscordConfigAssets,
    pub buttons: DiscordButtons,
    pub client_id: u64,
    pub state: String,
    pub details: String,
}

impl Default for DiscordConfig {
    fn default() -> Self {
        DiscordConfig {
            assets: DiscordConfigAssets {
                large_image: "".to_owned(),
                large_text: "".to_owned(),
                small_image: "".to_owned(),
                small_text: "".to_owned(),
            },
            buttons: DiscordButtons {
                btn1_text: "".to_owned(),
                btn1_url: "".to_owned(),
                btn2_text: "".to_owned(),
                btn2_url: "".to_owned(),
            },
            client_id: 0,
            details: "".to_owned(),
            state: "".to_owned(),
        }
    }
}

impl DiscordConfig {
    pub fn replace_templates(&mut self, template_hashmap: &HashMap<&str, String>) {
        let config: DiscordConfig = self.to_owned();

        self.assets.large_image =
            replace_template_variables(&template_hashmap, config.assets.large_image);
        self.assets.large_text =
            replace_template_variables(&template_hashmap, config.assets.large_text);
        self.assets.small_image =
            replace_template_variables(&template_hashmap, config.assets.small_image);
        self.assets.small_text =
            replace_template_variables(&template_hashmap, config.assets.small_text);

        self.buttons.btn1_text =
            replace_template_variables(&template_hashmap, config.buttons.btn1_text);
        self.buttons.btn1_url =
            replace_template_variables(&template_hashmap, config.buttons.btn1_url);
        self.buttons.btn2_text =
            replace_template_variables(&template_hashmap, config.buttons.btn2_text);
        self.buttons.btn2_url =
            replace_template_variables(&template_hashmap, config.buttons.btn2_url);

        self.details = replace_template_variables(&template_hashmap, config.details);
        self.state = replace_template_variables(&template_hashmap, config.state);
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct DiscordConfigAssets {
    pub large_image: String,
    pub large_text: String,
    pub small_image: String,
    pub small_text: String,
}

impl DiscordConfigAssets {
    pub fn is_empty(&self) -> bool {
        self.large_image.is_empty()
            && self.large_text.is_empty()
            && self.small_image.is_empty()
            && self.small_text.is_empty()
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct DiscordButtons {
    pub btn1_text: String,
    pub btn1_url: String,
    pub btn2_text: String,
    pub btn2_url: String,
}

impl DiscordButtons {
    pub fn is_empty(&self) -> bool {
        self.btn1_text.is_empty()
            && self.btn1_url.is_empty()
            && self.btn2_text.is_empty()
            && self.btn2_url.is_empty()
    }

    pub fn btn1_is_empty(&self) -> bool {
        self.btn1_text.is_empty() || self.btn1_url.is_empty()
    }

    pub fn btn2_is_empty(&self) -> bool {
        self.btn2_text.is_empty() || self.btn2_url.is_empty()
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SpotifyConfig {
    pub client_id: String,
    pub client_secret: String,
    pub refresh_token: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ProcessesConfig {
    pub idle_image: String,
    pub idle_text: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub processes: Vec<ProcessConfig>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ProcessConfig {
    pub image: String,
    pub name: String,
    pub text: String,
}
