use crate::{parser::variables::replace_template_variables, prelude::*};
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
pub fn initialize_config(overwrite: bool) -> Result<Config, Box<dyn Error>> {
    let file_path: &str = &file_path();
    debug!("Config file path: {file_path}");
    if Path::new(file_path).exists() {
        trace!("Config file found");
        return match read_config_file(overwrite) {
            Err(error) => {
                eprintln!("Invalid configuration file found. Use --config-overwrite to overwrite the invalid config file with default values.");
                Err(error)
            }
            Ok(config) => Ok(config),
        };
    } else {
        warn!("Config file not found, creating new file with defaults");
        let default: Config = Config::default();
        write_config(&default)?;
        return Ok(default);
    }
}

/// Write config to the file at `file_path()`
#[instrument(skip_all)]
pub fn write_config<T: SerializeConfig>(config: &T) -> Result<(), Box<dyn Error>> {
    let config_dir: String = dir_path();
    let config_file: String = file_path();

    let serialized_config: String = to_string(&config.get_whole_config())?;
    trace!("Serialized config");

    if !Path::new(&config_dir).exists() {
        fs::create_dir_all(&config_dir)?;
        trace!("Created config directory {config_dir}");
    }

    fs::write(&config_file, serialized_config)?;
    trace!("Wrote to file {config_file}");
    return Ok(());
}

/// Attempt to read and deserialize config from file. If an error occurs while deserializing, a default `Config` will be created.<br/>
/// If `overwrite` is set to false, an `Err(())` will be returned instead.
#[instrument(skip_all)]
pub fn read_config_file(overwrite: bool) -> Result<Config, Box<dyn Error>> {
    let config_path: String = file_path();
    let config_vector: Vec<u8> = fs::read(&config_path)?;
    debug!("Successfully read config file from {config_path}");
    return verify_config_integrity(config_vector, config_path, overwrite);
}

/// Deserialize string into `Config`. If an error occurs while deserializing, a default `Config` will be created.
/// If `overwrite` is set to false, an `Err(())` will be returned instead.
#[instrument(skip_all)]
fn verify_config_integrity(
    config_vector: Vec<u8>,
    config_path: String,
    overwrite: bool,
) -> Result<Config, Box<dyn Error>> {
    let config_string: String = String::from_utf8(config_vector)?;
    trace!("Successfully converted config file to utf8");

    return match from_str(&config_string) {
        Err(error) => {
            if !overwrite {
                return Err(Box::new(error));
            }
            warn!("Error while deserializing configuration file: {error}");

            fs::remove_file(config_path)?;
            info!("Removed invalid configuration file, creating new file with defaults");

            let default: Config = Config::default();
            write_config(&default)?;
            return Ok(default);
        }
        Ok(config) => {
            trace!("Config file validated");
            Ok(config)
        }
    };
}

pub trait SerializeConfig {
    fn get_whole_config(&self) -> Config;
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
            discord: DiscordConfig::new(1133837522074607749),
            processes: ProcessesConfig {
                idle_image: String::from("idle"),
                idle_text: String::from("Idle"),
                processes: vec![ProcessConfig {
                    image: String::from("code"),
                    name: String::from("code"),
                    text: String::from("Visual Studio Code"),
                }],
            },
            spotify: SpotifyConfig {
                client_id: String::new(),
                client_secret: String::new(),
                fallback: SpotifyFallbackConfig {
                    album_cover_url: String::new(),
                    album_name: String::new(),
                    artists: String::new(),
                    name: String::new(),
                    track_url: String::new(),
                },
                refresh_token: String::new(),
            },
        }
    }
}

impl SerializeConfig for Config {
    fn get_whole_config(&self) -> Config {
        return self.to_owned();
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

impl SerializeConfig for DiscordConfig {
    fn get_whole_config(&self) -> Config {
        return match read_config_file(false) {
            Err(_) => todo!(),
            Ok(mut config) => {
                config.discord = self.to_owned();
                config
            }
        };
    }
}

impl DiscordConfig {
    pub fn new(client_id: u64) -> Self {
        return Self {
            assets: DiscordConfigAssets {
                large_image: String::new(),
                large_text: String::new(),
                small_image: String::new(),
                small_text: String::new(),
            },
            buttons: DiscordButtons {
                btn1_text: String::new(),
                btn1_url: String::new(),
                btn2_text: String::new(),
                btn2_url: String::new(),
            },
            client_id,
            details: String::new(),
            state: String::new(),
        };
    }

    pub fn replace_templates(&mut self, template_hashmap: &HashMap<String, String>) {
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
    pub fallback: SpotifyFallbackConfig,
    pub refresh_token: String,
}

impl SerializeConfig for SpotifyConfig {
    fn get_whole_config(&self) -> Config {
        return match read_config_file(false) {
            Err(_) => todo!(),
            Ok(mut config) => {
                config.spotify = self.to_owned();
                config
            }
        };
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SpotifyFallbackConfig {
    pub album_name: String,
    #[serde(alias = "album_cover")]
    pub album_cover_url: String,
    pub artists: String,
    pub name: String,
    pub track_url: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ProcessesConfig {
    pub idle_image: String,
    pub idle_text: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub processes: Vec<ProcessConfig>,
}

impl SerializeConfig for ProcessesConfig {
    fn get_whole_config(&self) -> Config {
        return match read_config_file(false) {
            Err(_) => todo!(),
            Ok(mut config) => {
                config.processes = self.to_owned();
                config
            }
        };
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ProcessConfig {
    pub image: String,
    pub name: String,
    pub text: String,
}
