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
                large_image_key: "".to_string(),
                large_image_text: "".to_string(),
                small_image_key: "".to_string(),
                small_image_text: "".to_string(),
            },
            spotify: SpotifyConfig {
                client_id: "".to_string(),
                client_secret: "".to_string(),
                refresh_token: "".to_string(),
            },
            processes: ProcessesConfig {
                processes: vec![ProcessesProcessConfig {
                    display: "Visual Studio Code".to_string(),
                    name: "code".to_string(),
                }],
            },
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DiscordConfig {
    pub client_id: u64,
    pub state: String,
    pub details: String,
    pub large_image_key: String,
    pub large_image_text: String,
    pub small_image_key: String,
    pub small_image_text: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpotifyConfig {
    pub client_id: String,
    pub client_secret: String,
    pub refresh_token: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProcessesConfig {
    pub processes: Vec<ProcessesProcessConfig>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProcessesProcessConfig {
    pub display: String,
    pub name: String,
}
