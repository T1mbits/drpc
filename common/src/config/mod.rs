mod types;

use anyhow::Context;
use dirs::{cache_dir, config_dir};
use std::{fs, path::PathBuf};
use toml::{from_str, to_string};
pub use types::*;

fn config_path() -> PathBuf {
    match config_dir() {
        Some(dir) => PathBuf::from(format!("{}/ddrpc", dir.display())),
        None => PathBuf::from("./ddrpc"),
    }
}

fn config_file() -> PathBuf {
    config_path().join("ddrpc.toml")
}

pub fn cache_path() -> PathBuf {
    match cache_dir() {
        None => PathBuf::from("./ddrpc"),
        Some(dir) => dir.join("ddrpc"),
    }
}

pub fn cache_file() -> PathBuf {
    cache_path().join("ddrpc_spotify_token.json")
}

pub fn write_config(config: &Config) -> anyhow::Result<()> {
    let config_dir = config_path();
    let serialized_config = to_string(config).context("could not serialize config")?;

    if !config_dir.exists() {
        fs::create_dir_all(config_dir)?;
    }

    Ok(fs::write(config_file(), serialized_config)?)
}

pub fn get_config(overwrite: bool) -> anyhow::Result<Config> {
    let config_path = config_file();

    match from_str(
        &String::from_utf8(fs::read(&config_path).context(format!(
            "failed to read config from {}",
            config_path.display()
        ))?)
        .context(format!(
            "failed to deserialize config file at {}",
            config_path.display()
        ))?,
    ) {
        Ok(mut config) => {
            empty_config(&mut config);
            Ok(config)
        }
        Err(err) => {
            if overwrite {
                let config = Config::default();
                write_config(&config).context(format!(
                    "failed to overwrite config at {}",
                    config_path.display()
                ))?;
                return Ok(config);
            };

            Err(err.into())
        }
    }
}

/// Convert all empty values within the config to None.
pub fn empty_config(config: &mut Config) {
    empty_template(&mut config.activity)
}

fn empty_string(value: &mut Option<String>) {
    if value.as_ref().map_or(false, |s| s.is_empty()) {
        *value = None;
    }
}

/// Convert all empty strings and templates to None.
pub fn empty_template(activity: &mut ActivityTemplate) -> () {
    activity.foreach_field(empty_string);

    if let Some(assets) = &activity.assets {
        if assets.is_empty() {
            activity.assets = None;
        }
    }
    if let Some(buttons) = &activity.buttons {
        if buttons.is_empty() {
            activity.buttons = None;
        }
    }
}
