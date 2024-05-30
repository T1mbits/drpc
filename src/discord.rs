use crate::{
    parser::{variables::template_hashmap, CliDiscordSet},
    prelude::*,
    spotify,
};
use discord_rich_presence::{activity::*, DiscordIpc, DiscordIpcClient};
use rspotify::AuthCodeSpotify;
use std::collections::HashMap;

/// Bundle DiscordIpcClient and the Discord activity data associated to it.
// pub struct ClientBundle {
//     pub discord: DiscordIpcClient,
//     pub new_data: DiscordConfig,
//     pub spotify: AuthCodeSpotify,
// }

pub struct DiscordState {
    pub client: DiscordIpcClient,
    /// From [`DiscordConfig`] with all fields parsed with [`DiscordConfig::replace_templates`];
    pub prev_data: DiscordConfig,
}

impl DiscordState {
    fn new(client: DiscordIpcClient, client_id: u64) -> Self {
        Self {
            client,
            prev_data: DiscordConfig::new(client_id),
        }
    }
}

/// Print Discord activity data saved in config.
pub fn print_activity_data(config: &DiscordConfig) -> () {
    println!(
        "Client ID: {}\nDetails: {}\nState: {}\nLarge Image Key: {}\nLarge Image Text: {}\nSmall Image Key: {}\nSmall Image Text: {}\nButton 1 Text: {}\nButton 1 URL: {}\nButton 2 Text: {}\nButton 2 URL: {}",
        config.client_id,
        {if config.details.is_empty() {"<None>"} else {config.details.as_str()}},
        {if config.state.is_empty() {"<None>"} else {config.state.as_str()}},
        {if config.assets.large_image.is_empty() {"<None>"} else {config.assets.large_image.as_str()}},
        {if config.assets.large_text.is_empty() {"<None>"} else {config.assets.large_text.as_str()}},
        {if config.assets.small_image.is_empty() {"<None>"} else {config.assets.small_image.as_str()}},
        {if config.assets.small_text.is_empty() {"<None>"} else {config.assets.small_text.as_str()}},
        {if config.buttons.btn1_text.is_empty() {"<None>"} else {config.buttons.btn1_text.as_str()}},
        {if config.buttons.btn1_url.is_empty() {"<None>"} else {config.buttons.btn1_url.as_str()}},
        {if config.buttons.btn2_text.is_empty() {"<None>"} else {config.buttons.btn2_text.as_str()}},
        {if config.buttons.btn2_url.is_empty() {"<None>"} else {config.buttons.btn2_url.as_str()}},
    );
}

/// Overwrite Discord data in `Config` and write to file.
#[instrument(skip_all)]
pub fn set_activity_data(
    config: &mut DiscordConfig,
    arg: CliDiscordSet,
) -> Result<(), Box<dyn Error>> {
    trace!("Overwriting with:\n{arg:#?}");

    if let Some(id) = arg.client_id {
        config.client_id = id
    }
    if let Some(details) = arg.details {
        config.details = details
    }
    if !config.assets.is_empty() {
        if let Some(lik) = arg.large_image {
            config.assets.large_image = lik
        }
        if let Some(lit) = arg.large_text {
            config.assets.large_text = lit
        }
        if let Some(sik) = arg.small_image {
            config.assets.small_image = sik
        }
        if let Some(sit) = arg.small_text {
            config.assets.small_text = sit
        }
        if let Some(state) = arg.state {
            config.state = state
        }
    }
    if !config.buttons.is_empty() {
        if let Some(b1t) = arg.button1_text {
            config.buttons.btn1_text = b1t;
        }
        if let Some(b1u) = arg.button1_url {
            config.buttons.btn1_url = b1u;
        }
        if let Some(b2t) = arg.button2_text {
            config.buttons.btn2_text = b2t;
        }
        if let Some(b2u) = arg.button2_url {
            config.buttons.btn2_url = b2u;
        }
    }

    return write_config(config);
}

/// Initialize and connect `DiscordIpcClient`.
#[instrument(skip_all)]
pub async fn client_init(config: &mut Config) -> Result<AppState, Box<dyn Error>> {
    let mut client: DiscordIpcClient =
        DiscordIpcClient::new(&config.discord.client_id.to_string())?;
    trace!("Successfully initialized Discord client");

    client.connect()?;
    info!("Discord client connected to IPC");

    let spotify_client: Option<AuthCodeSpotify> = spotify::client_init(&mut config.spotify).await?;

    return Ok(AppState::new(DiscordState::new(client, 0), spotify_client));
}

/// Set Discord activity. Will clone `DiscordConfig` data and replace template variables before comparing to old data. If the new data matches<br/>
/// with the old data, the function will return. Otherwise, the new data is used and the activity will be overwritten.
#[instrument(skip_all)]
pub async fn set_activity(
    config: &Config,
    discord: &mut DiscordState,
    spotify: &Option<AuthCodeSpotify>,
) -> Result<(), Box<dyn Error>> {
    let mut new_data: DiscordConfig = config.discord.clone();
    trace!("Discord data cloned");

    let template_hashmap: HashMap<String, String> = template_hashmap(config, spotify).await;

    new_data.replace_templates(&template_hashmap);

    if new_data == discord.prev_data {
        trace!("Activity data has not changed");
        return Ok(());
    }

    info!("Activity data has changed, overwriting and setting activity");

    let mut activity: Activity = Activity::new();

    if !new_data.details.is_empty() {
        activity = activity.details(&new_data.details);
    }

    if !new_data.state.is_empty() {
        activity = activity.state(&new_data.state);
    }

    if !new_data.assets.is_empty() {
        let mut assets = Assets::new();

        if !new_data.assets.large_image.is_empty() {
            assets = assets.large_image(&new_data.assets.large_image);
        }

        if !new_data.assets.large_text.is_empty() {
            assets = assets.large_text(&new_data.assets.large_text)
        }

        if !new_data.assets.small_image.is_empty() {
            assets = assets.small_image(&new_data.assets.small_image);
        }

        if !new_data.assets.small_text.is_empty() {
            assets = assets.small_text(&new_data.assets.small_text);
        }

        activity = activity.assets(assets);
    }

    if !new_data.buttons.is_empty() {
        let mut buttons: Vec<Button> = Vec::new();

        if !new_data.buttons.btn1_is_empty() {
            buttons.push(Button::new(
                &new_data.buttons.btn1_text,
                &new_data.buttons.btn1_url,
            ));
        }

        if !new_data.buttons.btn2_is_empty() {
            buttons.push(Button::new(
                &new_data.buttons.btn2_text,
                &new_data.buttons.btn2_url,
            ));
        }

        activity = activity.buttons(buttons);
    }

    discord.client.set_activity(activity)?;

    debug!("Activity set to: \n{new_data:?}");
    discord.prev_data = new_data.to_owned();

    return Ok(());
}

/// Clears the current Discord activity
#[instrument(skip_all)]
pub fn clear_activity(client: &mut DiscordIpcClient) -> Result<(), Box<dyn Error>> {
    return client.clear_activity();
}

/// Updates `Config` and sets Discord activity if no errors occur during config reread. If an error does occur, a warning will be logged<br/>
/// but no changes will take place.
#[instrument(skip_all)]
pub async fn update_activity(
    config: &Config,
    discord: &mut DiscordState,
    spotify: &Option<AuthCodeSpotify>,
) -> Result<(), Box<dyn Error>> {
    // *config = match read_config_file(false) {
    //     Err(_) => {
    //         warn!("Config file was not deserialized. Will continue to use old config.");
    //         return Ok(client);
    //     }
    //     Ok(config) => config,
    // };
    trace!("Updating Discord activity");
    return set_activity(config, discord, spotify).await;
}
