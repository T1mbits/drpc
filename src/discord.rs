use std::io;

use discord_rpc_client::{models::ActivityAssets, Client};

use crate::{
    config::{read_config_file, write_config, DConfig, DiscordConfig},
    logging::ddrpc_log,
    parser::CliDiscordSet,
};

pub fn get(config: &DiscordConfig) {
    println!(
        "Client ID: {}\nDetails: {}\nState: {}\nLarge Image Key: {}\nLarge Image Text: {}\nSmall Image Key: {}\nSmall Image Text: {}",
        config.client_id,
        {if config.details.is_empty() {"<None>"} else {config.details.as_str()}},
        {if config.state.is_empty() {"<None>"} else {config.state.as_str()}},
        {if config.assets.large_image.is_empty() {"<None>"} else {config.assets.large_image.as_str()}},
        {if config.assets.large_text.is_empty() {"<None>"} else {config.assets.large_text.as_str()}},
        {if config.assets.small_image.is_empty() {"<None>"} else {config.assets.small_image.as_str()}},
        {if config.assets.small_text.is_empty() {"<None>"} else {config.assets.small_text.as_str()}},
    );
}

pub fn set(mut config: DConfig, arg: CliDiscordSet) {
    if let Some(id) = arg.client_id {
        config.discord.client_id = id
    }
    if let Some(details) = arg.details {
        config.discord.details = details
    }
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
    write_config(&config);
}

pub fn init(config: &DiscordConfig) -> Result<Client, io::Error> {
    let mut client: Client = Client::new(config.client_id);
    ddrpc_log("Attempting to start client");
    client.start();
    client.on_ready(|_context| {
        ddrpc_log("Discord Client ready!");
    });
    client.on_error(|context| ddrpc_log(&format!("Error: {:?}", context)));
    // match client.set_activity(|activity| activity.state("test")) {
    //     Ok(_) => {}
    //     Err(error) => ddrpc_log(&format!("Error while setting activity: {}", error)),
    // }
    // ddrpc_log("Client should have finished now");
    return Ok(client);
}

pub fn start(config: &DiscordConfig, client: &mut Client) {
    ddrpc_log(&format!("{:?}", config));
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
            ddrpc_log(&format!("{:?}", activity));
            activity
        })
        .unwrap();
}

pub fn stop(client: &mut Client) -> Result<bool, io::Error> {
    client.clear_activity().unwrap();
    Ok(true)
}

pub fn update(config: &mut DiscordConfig, client: &mut Client) -> () {
    *config = read_config_file().discord;
    ddrpc_log(&format!("{:?}", config));
    start(&config, client);
}
