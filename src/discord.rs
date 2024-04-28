use crate::{
    config::{read_config_file, write_config, Config, DiscordConfig},
    parser::CliDiscordSet,
};
use discord_rich_presence::{activity::*, DiscordIpc, DiscordIpcClient};
// use discord_rpc_client::{models::ActivityAssets, Client};
use std::{io::Error, process::exit};
use tracing::{debug, error, info, instrument, trace, warn};

pub fn get_activity_data(config: DiscordConfig) -> () {
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

pub fn set_activity_data(mut config: Config, arg: CliDiscordSet) -> () {
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
pub fn client_init(config: &DiscordConfig) -> DiscordIpcClient {
    let mut client = DiscordIpcClient::new(&config.client_id.to_string()).unwrap();

    match client.connect() {
        Err(err) => {
            error!("{err}");
            exit(1)
        }
        Ok(_) => info!("Discord Client connected to IPC"),
    }

    client
}

#[instrument(skip_all)]
pub fn set_activity(config: &DiscordConfig, mut client: DiscordIpcClient) -> DiscordIpcClient {
    debug!("Setting Discord activity to:\n{config:#?}");

    let mut activity = Activity::new().state("state");

    if !config.details.is_empty() {
        activity = activity.details(&config.details);
    }

    if !config.state.is_empty() {
        activity = activity.state(&config.state);
    }

    if !config.assets.is_empty() {
        let mut assets = Assets::new();

        if !config.assets.large_image.is_empty() {
            assets = assets.large_image(&config.assets.large_image);
        }

        if !config.assets.large_text.is_empty() {
            assets = assets.large_text(&config.assets.large_text);
        }

        if !config.assets.small_image.is_empty() {
            assets = assets.small_image(&config.assets.small_image);
        }

        if !config.assets.small_text.is_empty() {
            assets = assets.small_text(&config.assets.small_text);
        }

        activity = activity.assets(assets);
    }

    if !config.buttons.is_empty() {
        let mut buttons: Vec<Button> = Vec::new();

        if !config.buttons.btn1_is_empty() {
            buttons.push(Button::new(
                &config.buttons.btn1_text,
                &config.buttons.btn1_url,
            ));
        }

        if !config.buttons.btn2_is_empty() {
            buttons.push(Button::new(
                &config.buttons.btn2_text,
                &config.buttons.btn2_url,
            ));
        }

        activity = activity.buttons(buttons);
    }

    client.set_activity(activity).unwrap();
    client
}

#[instrument(skip_all)]
pub fn clear_activity(mut client: DiscordIpcClient) -> Result<DiscordIpcClient, Error> {
    client.clear_activity().unwrap();
    info!("Discord RPC activity cleared");
    Ok(client)
}

#[instrument(skip_all)]
pub fn update_activity(config: &mut DiscordConfig, client: DiscordIpcClient) -> DiscordIpcClient {
    *config = read_config_file().discord;
    trace!("Discord config:\n{config:#?}");
    info!("Updating Discord RPC");
    set_activity(&config, client)
}
