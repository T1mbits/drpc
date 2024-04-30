use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::discord::replace_variables;

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
                idle_icon: "idle".to_string(),
                idle_text: "Idle".to_string(),
                process: vec![ProcessesProcessConfig {
                    icon: "code".to_string(),
                    process: "code".to_string(),
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
    pub fn replace_templates(mut self, template_hashmap: &HashMap<&str, String>) -> Self {
        self.assets.large_image = replace_variables(&template_hashmap, self.assets.large_image);
        self.assets.large_text = replace_variables(&template_hashmap, self.assets.large_text);
        self.assets.small_image = replace_variables(&template_hashmap, self.assets.small_image);
        self.assets.small_text = replace_variables(&template_hashmap, self.assets.small_text);

        self.buttons.btn1_text = replace_variables(&template_hashmap, self.buttons.btn1_text);
        self.buttons.btn1_url = replace_variables(&template_hashmap, self.buttons.btn1_url);
        self.buttons.btn2_text = replace_variables(&template_hashmap, self.buttons.btn2_text);
        self.buttons.btn2_url = replace_variables(&template_hashmap, self.buttons.btn2_url);

        self.details = replace_variables(&template_hashmap, self.details);
        self.state = replace_variables(&template_hashmap, self.state);

        self
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
    pub idle_icon: String,
    pub idle_text: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub process: Vec<ProcessesProcessConfig>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ProcessesProcessConfig {
    pub icon: String,
    pub process: String,
    pub text: String,
}
