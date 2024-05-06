use crate::{
    parser::CliDiscordSet,
    prelude::*,
    processes::{get_active_data, get_names},
    spotify::{self, get_currently_playing_track, TrackData},
};
use discord_rich_presence::{activity::*, DiscordIpc, DiscordIpcClient};
use rspotify::AuthCodeSpotify;
use std::collections::HashMap;

/// Bundle DiscordIpcClient and the Discord activity data associated to it.
pub struct ClientBundle {
    pub discord: DiscordIpcClient,
    pub replaced_data: DiscordConfig,
    pub spotify: AuthCodeSpotify,
}

/// Print Discord activity data saved in config.
pub fn print_activity_data(config: &DiscordConfig) -> Result<Option<ClientBundle>, ()> {
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
    return Ok(None);
}

/// Overwrite Discord data in `Config` and write to file.
#[instrument(skip_all)]
pub fn set_activity_data(
    config: &mut Config,
    arg: CliDiscordSet,
) -> Result<Option<ClientBundle>, ()> {
    trace!("Overwriting with:\n{arg:#?}");

    if let Some(id) = arg.client_id {
        config.discord.client_id = id
    }
    if let Some(details) = arg.details {
        config.discord.details = details
    }
    if !config.discord.assets.is_empty() {
        if let Some(lik) = arg.large_image {
            config.discord.assets.large_image = lik
        }
        if let Some(lit) = arg.large_text {
            config.discord.assets.large_text = lit
        }
        if let Some(sik) = arg.small_image {
            config.discord.assets.small_image = sik
        }
        if let Some(sit) = arg.small_text {
            config.discord.assets.small_text = sit
        }
        if let Some(state) = arg.state {
            config.discord.state = state
        }
    }
    if !config.discord.buttons.is_empty() {
        if let Some(b1t) = arg.button1_text {
            config.discord.buttons.btn1_text = b1t;
        }
        if let Some(b1u) = arg.button1_url {
            config.discord.buttons.btn1_url = b1u;
        }
        if let Some(b2t) = arg.button2_text {
            config.discord.buttons.btn2_text = b2t;
        }
        if let Some(b2u) = arg.button2_url {
            config.discord.buttons.btn2_url = b2u;
        }
    }

    return write_config(&config);
}

/// Initialize and connect `DiscordIpcClient`.
#[instrument(skip_all)]
pub async fn client_init(config: &mut Config) -> Result<ClientBundle, ()> {
    let mut client: DiscordIpcClient =
        match DiscordIpcClient::new(&config.discord.client_id.to_string()) {
            Err(error) => {
                error!("Unable to initialize Discord client: {error}");
                return Err(());
            }
            Ok(client) => {
                trace!("Successfully initialized Discord client");
                client
            }
        };

    match client.connect() {
        Err(err) => {
            error!("Error while connect Discord client to IPC: {err}");
            return Err(());
        }
        Ok(_) => info!("Discord client connected to IPC"),
    }

    let spotify_client = match spotify::client_init(config).await {
        Err(_) => return Err(()),
        Ok(client) => client,
    };

    return Ok(ClientBundle {
        discord: client,
        replaced_data: DiscordConfig::new(0),
        spotify: spotify_client,
    });
}

/// Create the hashmap for template variables and their replacements in Discord data.
#[instrument(skip_all)]
async fn template_hashmap<'th>(
    config: &Config,
    client: &AuthCodeSpotify,
) -> HashMap<&'th str, String> {
    let processes = get_names(&config.processes);

    let (process_text, process_icon) = get_active_data(&config.processes, &processes);
    let track = match get_currently_playing_track(client).await.unwrap() {
        None => TrackData::fallback(&config.spotify.fallback),
        Some(track_data) => track_data,
    };

    let mut replace_hashmap: HashMap<&str, String> = HashMap::new();
    replace_hashmap.insert("process.icon", process_icon);
    replace_hashmap.insert("process.text", process_text);
    replace_hashmap.insert("idle.icon", config.processes.idle_image.to_owned());
    replace_hashmap.insert("idle.text", config.processes.idle_text.to_owned());

    replace_hashmap.insert("spotify.track.name", track.name);
    replace_hashmap.insert("spotify.track.artists", track.artists);
    replace_hashmap.insert("spotify.track.url", track.track_url);
    replace_hashmap.insert("spotify.album.cover", track.album_cover_url);
    replace_hashmap.insert("spotify.album.name", track.album_name);

    trace!("Template variable hashmap created");
    return replace_hashmap;
}

/// Replace recognized template variables with their corresponding data.
#[instrument(skip_all)]
pub fn replace_template_variables(
    template_hashmap: &HashMap<&str, String>,
    mut string: String,
) -> String {
    if string.is_empty() || !string.contains("{{") {
        trace!("String does not contain template variables");
        return string;
    }

    // let nestable_template_variables = vec!["spotify"];

    for (target, replacement) in template_hashmap {
        let target = format!("{{{{{}}}}}", target);
        // trace!("Replacing template variable \"{target}\" with \"{replacement}\"");
        string = string.replace(&target, replacement);
    }

    return string;
}

/// Set Discord activity. Will clone `DiscordConfig` data and replace template variables before comparing to old data. If the new data matches<br/>
/// with the old data, the function will return. Otherwise, the new data is used and the activity will be overwritten.
#[instrument(skip_all)]
pub async fn set_activity(
    mut bundle: ClientBundle,
    config: &mut Config,
) -> Result<ClientBundle, ()> {
    let mut replaced_data: DiscordConfig = config.discord.clone();
    trace!("Discord data cloned");

    let template_hashmap: HashMap<&str, String> = template_hashmap(config, &bundle.spotify).await;

    info!("{template_hashmap:#?}");

    replaced_data.replace_templates(&template_hashmap);

    if replaced_data == bundle.replaced_data {
        debug!("Activity data has not changed");
        return Ok(bundle);
    }

    trace!("Activity data has changed, overwriting and setting activity");

    bundle.replaced_data = replaced_data;

    let mut activity = Activity::new();

    if !config.discord.details.is_empty() {
        activity = activity.details(&bundle.replaced_data.details);
    }

    if !config.discord.state.is_empty() {
        activity = activity.state(&bundle.replaced_data.state);
    }

    if !config.discord.assets.is_empty() {
        let mut assets = Assets::new();

        if !config.discord.assets.large_image.is_empty() {
            assets = assets.large_image(&bundle.replaced_data.assets.large_image);
        }

        if !config.discord.assets.large_text.is_empty() {
            assets = assets.large_text(&bundle.replaced_data.assets.large_text)
        }

        if !config.discord.assets.small_image.is_empty() {
            assets = assets.small_image(&bundle.replaced_data.assets.small_image);
        }

        if !config.discord.assets.small_text.is_empty() {
            assets = assets.small_text(&bundle.replaced_data.assets.small_text);
        }

        activity = activity.assets(assets);
    }

    if !config.discord.buttons.is_empty() {
        let mut buttons: Vec<Button> = Vec::new();

        if !config.discord.buttons.btn1_is_empty() {
            buttons.push(Button::new(
                &bundle.replaced_data.buttons.btn1_text,
                &bundle.replaced_data.buttons.btn1_url,
            ));
        }

        if !config.discord.buttons.btn2_is_empty() {
            buttons.push(Button::new(
                &bundle.replaced_data.buttons.btn2_text,
                &bundle.replaced_data.buttons.btn2_url,
            ));
        }

        activity = activity.buttons(buttons);
    }

    let data: DiscordConfig = bundle.replaced_data.to_owned();
    trace!("Activity set to: \n{data:#?}");

    return match bundle.discord.set_activity(activity) {
        Err(error) => {
            error!("Error while setting activity: {error}");
            Err(())
        }
        Ok(_) => Ok(bundle),
    };
}

/// Clears the current Discord activity
#[instrument(skip_all)]
pub fn clear_activity(mut bundle: ClientBundle) -> Result<ClientBundle, ()> {
    return match bundle.discord.clear_activity() {
        Err(error) => {
            error!("Error while clearing Discord activity: {error}");
            Err(())
        }
        Ok(_) => {
            info!("Discord activity cleared");
            Ok(bundle)
        }
    };
}

/// Updates `Config` and sets Discord activity if no errors occur during config reread. If an error does occur, a warning will be logged<br/>
/// but no changes will take place.
#[instrument(skip_all)]
pub async fn update_activity(
    config: &mut Config,
    client: ClientBundle,
) -> Result<ClientBundle, ()> {
    *config = match read_config_file(false) {
        Err(_) => {
            warn!("Config file was not deserialized. Will continue to use old config.");
            return Ok(client);
        }
        Ok(config) => config,
    };
    info!("Updating Discord activity");
    return match set_activity(client, config).await {
        Err(_) => Err(()),
        Ok(client) => Ok(client),
    };
}
