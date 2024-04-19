use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct DConfig {
    pub discord: DiscordConfig,
    pub spotify: SpotifyConfig,
    pub processes: ProcessesConfig,
}

impl Default for DConfig {
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
            },
            spotify: SpotifyConfig {
                client_id: "".to_string(),
                client_secret: "".to_string(),
                refresh_token: "".to_string(),
            },
            processes: ProcessesConfig {
                process: vec![
                    ProcessesProcessConfig {
                        display: "Visual Studio Code".to_string(),
                        name: "code".to_string(),
                    },
                    ProcessesProcessConfig {
                        display: "Firefox".to_string(),
                        name: "firefox".to_string(),
                    },
                ],
            },
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DiscordConfig {
    pub client_id: u64,
    pub state: String,
    pub details: String,
    pub assets: DiscordConfigAssets,
}

#[derive(Debug, Serialize, Deserialize)]
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

#[derive(Debug, Serialize, Deserialize)]
pub struct SpotifyConfig {
    pub client_id: String,
    pub client_secret: String,
    pub refresh_token: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProcessesConfig {
    pub process: Vec<ProcessesProcessConfig>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProcessesProcessConfig {
    pub display: String,
    pub name: String,
}
