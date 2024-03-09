use std::{
    io,
    sync::mpsc::{channel, RecvTimeoutError, Sender},
    thread, time,
};

use crate::{config::structure::DiscordConfig, logging::ddrpc_log};

pub enum DiscordThreadCommands {
    Connect,
    Disconnect,
    Get,
    Set,
}

pub fn discord_thread(
    config: DiscordConfig,
    sender_main: Sender<Vec<u8>>,
) -> Result<Sender<DiscordThreadCommands>, io::Error> {
    let (sender_discord, receiver_discord) = channel();

    match thread::Builder::new()
        .name("discord_rpc".to_owned())
        .spawn(move || {
            let config: DiscordConfig = config;
            let sender_main: Sender<Vec<u8>> = sender_main;
            loop {
                let discord_channel: DiscordThreadCommands =
                    match receiver_discord.recv_timeout(time::Duration::from_secs(5)) {
                        Err(error) if error == RecvTimeoutError::Timeout => {
                            continue;
                        }
                        Err(error) => {
                            ddrpc_log(&format!("All senders disconnected: {error}"));
                            ddrpc_log("Closing Discord RPC thread");
                            break;
                        }
                        Ok(str) => str,
                    };
                match discord_channel {
                    DiscordThreadCommands::Connect => todo!(),
                    DiscordThreadCommands::Disconnect => todo!(),
                    DiscordThreadCommands::Get => match sender_main.send(get_command(&config)) {
                        Err(error) => {
                            ddrpc_log(&format!("Main thread's receiver crashed: {error}"));
                            ddrpc_log("Closing Discord RPC thread");
                            break;
                        }
                        Ok(_) => (),
                    },
                    DiscordThreadCommands::Set => todo!(),
                }
            }
        }) {
        Err(error) => return Err(error),
        Ok(_) => (),
    }

    Ok(sender_discord)
}

fn get_command(config: &DiscordConfig) -> Vec<u8> {
    format!(
        "Client ID: {}\nDetails: {}\nLarge Image Key: {}\nLarge Image Text: {}\nSmall Image Key: {}\nSmall Image Text: {}\nState: {}",
        config.client_id,
        {if config.details == "" {"<None>"} else {config.details.as_str()}},
        {if config.large_image_key == "" {"<None>"} else {config.large_image_key.as_str()}},
        {if config.large_image_text == "" {"<None>"} else {config.large_image_text.as_str()}},
        {if config.small_image_key == "" {"<None>"} else {config.small_image_key.as_str()}},
        {if config.small_image_text == "" {"<None>"} else {config.small_image_text.as_str()}},
        {if config.state == "" {"<None>"} else {config.state.as_str()}}
    )
    .as_bytes()
    .to_owned()
}
