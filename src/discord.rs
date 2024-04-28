use crate::{
    config::{read_config_file, write_config, Config, DiscordConfig},
    parser::CliDiscordSet,
};
use discord_rpc_client::{models::ActivityAssets, Client};
use std::io::Error;
use tracing::{debug, info, instrument, trace, warn};

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
pub fn client_init(config: &DiscordConfig) -> Result<Client, Error> {
    let mut client: Client = Client::new(config.client_id);
    info!("Starting Discord Client");
    client.start();

    client.on_ready(move |_context| {
        info!("Discord Client ready!");
    });

    client.on_error(move |context| warn!("Error: {context:#?}"));

    return Ok(client);
}

#[instrument(skip_all)]
pub fn set_activity(config: &DiscordConfig, client: &mut Client) -> () {
    debug!("Setting Discord activity to:\n{config:#?}");
    client
        .set_activity(|activity| {
            let mut activity = activity;

            if !config.details.is_empty() {
                activity.details = Some(config.details.to_owned());
            }

            if !config.state.is_empty() {
                activity.state = Some(config.state.to_owned());
            }

            if !config.assets.is_empty() {
                let mut assets = ActivityAssets::new();

                if !config.assets.large_image.is_empty() {
                    assets.large_image = Some(config.assets.large_image.to_owned());
                }

                if !config.assets.large_text.is_empty() {
                    assets.large_text = Some(config.assets.large_text.to_owned());
                }

                if !config.assets.small_image.is_empty() {
                    assets.small_image = Some(config.assets.small_image.to_owned());
                }

                if !config.assets.small_text.is_empty() {
                    assets.small_text = Some(config.assets.small_text.to_owned());
                }
                activity.assets = Some(assets);
            }

            trace!("Discord activity set to:\n{activity:#?}");
            activity
        })
        .unwrap();
}

#[instrument(skip_all)]
pub fn clear_activity(client: &mut Client) -> Result<bool, Error> {
    client.clear_activity().unwrap();
    info!("Discord RPC activity cleared");
    Ok(true)
}

#[instrument(skip_all)]
pub fn update_activity(config: &mut DiscordConfig, client: &mut Client) -> () {
    *config = read_config_file().discord;
    trace!("Discord config:\n{config:#?}");
    info!("Updating Discord RPC");
    set_activity(&config, client);
}
