use crate::{
    config::{read_config_file, write_config, Config, DiscordConfig},
    parser::CliDiscordSet,
    processes::{get_processes, replace_asset},
};
use discord_rich_presence::{activity::*, DiscordIpc, DiscordIpcClient};
// use discord_rpc_client::{models::ActivityAssets, Client};
use std::{collections::HashMap, io::Error, process::exit};
use tracing::{error, info, instrument, trace, warn};

pub struct DiscordClientWrapper {
    client: DiscordIpcClient,
    replaced_data: DiscordConfig,
}

pub fn get_activity_data(config: &DiscordConfig) -> () {
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

pub fn set_activity_data(config: &mut Config, arg: CliDiscordSet) -> () {
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

    write_config(&config);
}

#[instrument(skip_all)]
pub fn client_init(client_id: u64) -> DiscordClientWrapper {
    let mut client = DiscordIpcClient::new(&client_id.to_string()).unwrap();

    match client.connect() {
        Err(err) => {
            error!("{err}");
            exit(1)
        }
        Ok(_) => info!("Discord Client connected to IPC"),
    }

    return DiscordClientWrapper {
        client,
        replaced_data: DiscordConfig::default(),
    };
}

#[instrument(skip_all)]
fn template_hashmap(config: &Config) -> HashMap<&str, String> {
    let processes = get_processes(&config.processes);

    let mut replace_hashmap: HashMap<&str, String> = HashMap::new();
    replace_hashmap.insert(
        "process.text",
        replace_asset(&config.processes, &processes).0,
    );
    replace_hashmap.insert(
        "process.icon",
        replace_asset(&config.processes, &processes).1,
    );

    return replace_hashmap;
}

#[instrument(skip_all)]
pub fn replace_variables(template_hashmap: &HashMap<&str, String>, mut string: String) -> String {
    if string.is_empty() {
        return string;
    }
    for (target, replacement) in template_hashmap {
        let target = format!("{{{{{}}}}}", target);
        string = string.replace(&target, replacement)
    }

    return string;
}

#[instrument(skip_all)]
pub fn set_activity(
    mut client_wrapper: DiscordClientWrapper,
    config: &mut Config,
) -> DiscordClientWrapper {
    let mut replaced_data: DiscordConfig = config.discord.clone();

    let template_hashmap: HashMap<&str, String> = template_hashmap(config);
    replaced_data = replaced_data.replace_templates(&template_hashmap);

    if replaced_data == client_wrapper.replaced_data {
        return client_wrapper;
    }

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

    client_wrapper.client.set_activity(activity).unwrap();
    return client_wrapper;
}

#[instrument(skip_all)]
pub fn clear_activity(mut client: DiscordIpcClient) -> Result<DiscordIpcClient, Error> {
    client.clear_activity().unwrap();
    info!("Discord RPC activity cleared");
    return Ok(client);
}

#[instrument(skip_all)]
pub fn update_activity(config: &mut Config, client: DiscordClientWrapper) -> DiscordClientWrapper {
    *config = read_config_file();
    info!("Updating Discord RPC");
    let client: DiscordClientWrapper = set_activity(client, config);
    info!("Updated Discord RPC");
    return client;
}
