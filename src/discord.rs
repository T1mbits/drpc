use crate::{
    config::{read_config_file, write_config, Config, DiscordConfig},
    parser::CliDiscordSet,
    processes::{get_active_data, get_names},
};
use discord_rich_presence::{activity::*, DiscordIpc, DiscordIpcClient};
use std::collections::HashMap;
use tracing::{debug, error, info, instrument, trace, warn};

/// Bundle DiscordIpcClient and the Discord activity data associated to it.
pub struct DiscordClientWrapper {
    client: DiscordIpcClient,
    replaced_data: DiscordConfig,
}

/// Print Discord activity data saved in config.
pub fn print_activity_data(config: &DiscordConfig) -> Result<Option<DiscordClientWrapper>, ()> {
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
) -> Result<Option<DiscordClientWrapper>, ()> {
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
pub fn client_init(client_id: u64) -> Result<DiscordClientWrapper, ()> {
    let mut client: DiscordIpcClient = match DiscordIpcClient::new(&client_id.to_string()) {
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

    return Ok(DiscordClientWrapper {
        client,
        replaced_data: DiscordConfig::default(),
    });
}

/// Create the hashmap for template variables and their replacements in Discord data.
#[instrument(skip_all)]
fn template_hashmap(config: &Config) -> HashMap<&str, String> {
    let processes = get_names(&config.processes);

    let (process_text, process_icon) = get_active_data(&config.processes, &processes);

    let mut replace_hashmap: HashMap<&str, String> = HashMap::new();
    replace_hashmap.insert("process.icon", process_icon);
    replace_hashmap.insert("process.text", process_text);
    replace_hashmap.insert("idle.icon", config.processes.idle_image.to_owned());
    replace_hashmap.insert("idle.text", config.processes.idle_text.to_owned());

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
    for (target, replacement) in template_hashmap {
        let target = format!("{{{{{}}}}}", target);
        // trace!("Replacing template variable \"{target}\" with \"{replacement}\"");
        string = string.replace(&target, replacement)
    }

    return string;
}

/// Set Discord activity. Will clone `DiscordConfig` data and replace template variables before comparing to old data. If the new data matches<br/>
/// with the old data, the function will return. Otherwise, the new data is used and the activity will be overwritten.
#[instrument(skip_all)]
pub fn set_activity(
    mut client_wrapper: DiscordClientWrapper,
    config: &mut Config,
) -> Result<DiscordClientWrapper, ()> {
    let mut replaced_data: DiscordConfig = config.discord.clone();
    trace!("Discord data cloned");

    let template_hashmap: HashMap<&str, String> = template_hashmap(config);
    replaced_data.replace_templates(&template_hashmap);

    if replaced_data == client_wrapper.replaced_data {
        debug!("Activity data has not changed");
        return Ok(client_wrapper);
    }

    trace!("Activity data has changed, overwriting and setting activity");

    client_wrapper.replaced_data = replaced_data;

    let mut activity = Activity::new();

    if !config.discord.details.is_empty() {
        activity = activity.details(&client_wrapper.replaced_data.details);
    }

    if !config.discord.state.is_empty() {
        activity = activity.state(&client_wrapper.replaced_data.state);
    }

    if !config.discord.assets.is_empty() {
        let mut assets = Assets::new();

        if !config.discord.assets.large_image.is_empty() {
            assets = assets.large_image(&client_wrapper.replaced_data.assets.large_image);
        }

        if !config.discord.assets.large_text.is_empty() {
            assets = assets.large_text(&client_wrapper.replaced_data.assets.large_text)
        }

        if !config.discord.assets.small_image.is_empty() {
            assets = assets.small_image(&client_wrapper.replaced_data.assets.small_image);
        }

        if !config.discord.assets.small_text.is_empty() {
            assets = assets.small_text(&client_wrapper.replaced_data.assets.small_text);
        }

        activity = activity.assets(assets);
    }

    if !config.discord.buttons.is_empty() {
        let mut buttons: Vec<Button> = Vec::new();

        if !config.discord.buttons.btn1_is_empty() {
            buttons.push(Button::new(
                &client_wrapper.replaced_data.buttons.btn1_text,
                &client_wrapper.replaced_data.buttons.btn1_url,
            ));
        }

        if !config.discord.buttons.btn2_is_empty() {
            buttons.push(Button::new(
                &client_wrapper.replaced_data.buttons.btn2_text,
                &client_wrapper.replaced_data.buttons.btn2_url,
            ));
        }

        activity = activity.buttons(buttons);
    }

    let data: DiscordConfig = client_wrapper.replaced_data.to_owned();
    trace!("Activity set to: \n{data:#?}");

    return match client_wrapper.client.set_activity(activity) {
        Err(error) => {
            error!("Error while setting activity: {error}");
            Err(())
        }
        Ok(_) => Ok(client_wrapper),
    };
}

/// Clears the current Discord activity
#[instrument(skip_all)]
pub fn clear_activity(mut wrapper: DiscordClientWrapper) -> Result<DiscordClientWrapper, ()> {
    return match wrapper.client.clear_activity() {
        Err(error) => {
            error!("Error while clearing Discord activity: {error}");
            Err(())
        }
        Ok(_) => {
            info!("Discord activity cleared");
            Ok(wrapper)
        }
    };
}

/// Updates `Config` and sets Discord activity if no errors occur during config reread. If an error does occur, a warning will be logged<br/>
/// but no changes will take place.
#[instrument(skip_all)]
pub fn update_activity(
    config: &mut Config,
    client: DiscordClientWrapper,
) -> Result<DiscordClientWrapper, ()> {
    *config = match read_config_file(false) {
        Err(_) => {
            warn!("Config file was not deserialized. Will continue to use old config.");
            return Ok(client);
        }
        Ok(config) => config,
    };
    info!("Updating Discord activity");
    return match set_activity(client, config) {
        Err(_) => Err(()),
        Ok(client) => Ok(client),
    };
}
