use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Config {
    pub discord: DiscordConfig,
    pub spotify: SpotifyConfig,
    pub processes: ProcessesConfig,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            discord: DiscordConfig {
                client_id: 1133837522074607749,
                state: "".to_string(),
                details: "".to_string(),
                assets: DiscordConfigAssets {
                    large_image: "".to_string(),
                    large_text: "".to_string(),
                    small_image: "".to_string(),
                    small_text: "".to_string(),
                },
                buttons: DiscordButtons {
                    btn1_text: "Repository".to_string(),
                    btn1_url: "https://github.com/T1mbits/ddrpc".to_string(),
                    btn2_text: "".to_string(),
                    btn2_url: "".to_string(),
                },
            },
            spotify: SpotifyConfig {
                client_id: "".to_string(),
                client_secret: "".to_string(),
                refresh_token: "".to_string(),
            },
            processes: ProcessesConfig {
                process: vec![
                    ProcessesProcessConfig {
                        icon: "code".to_string(),
                        process: "code".to_string(),
                        text: "Visual Studio Code".to_string(),
                    },
                    ProcessesProcessConfig {
                        icon: "firefox".to_string(),
                        process: "firefox".to_string(),
                        text: "Firefox".to_string(),
                    },
                ],
            },
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DiscordConfig {
    pub client_id: u64,
    pub state: String,
    pub details: String,
    pub assets: DiscordConfigAssets,
    pub buttons: DiscordButtons,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
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

#[derive(Clone, Debug, Serialize, Deserialize)]
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
    pub process: Vec<ProcessesProcessConfig>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ProcessesProcessConfig {
    pub icon: String,
    pub process: String,
    pub text: String,
}
