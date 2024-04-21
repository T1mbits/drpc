use crate::{config::DiscordConfig, discord::*, logging::ddrpc_log};

use discord_rpc_client::Client;
use std::{
    io::{self}, // ErrorKind},
    // process,
    sync::mpsc::*,
    thread,
    time,
};

pub enum DiscordThreadCommands {
    Connect,
    Disconnect,
    // Get,
    Update,
}

pub fn discord_thread(
    config: DiscordConfig,
    sender_main: Sender<Vec<u8>>,
) -> Result<Sender<DiscordThreadCommands>, io::Error> {
    let (sender_discord, receiver_discord) = channel();

    match thread::Builder::new()
        .name("discord_rpc".to_owned())
        .spawn(move || {
            let mut config: DiscordConfig = config;
            let _sender_main: Sender<Vec<u8>> = sender_main;
            let mut client: Option<Client> = None;
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
                    DiscordThreadCommands::Connect => {
                        if let Some(c) = &mut client {
                            start(&config, c);
                        } else {
                            client = match init(&config) {
                                Ok(mut client) => {
                                    start(&config, &mut client);
                                    Some(client)
                                }
                                Err(error) => panic!("{error}"),
                            };
                        }
                    }
                    DiscordThreadCommands::Disconnect => {
                        if let Some(c) = &mut client {
                            ddrpc_log("some");
                            stop(c).unwrap();
                        }
                        ()
                    }
                    // DiscordThreadCommands::Get => match sender_main.send(get(&config)) {
                    //     Err(error) => {
                    //         ddrpc_log(&format!("Main thread's receiver crashed: {error}"));
                    //         ddrpc_log("Closing Discord RPC thread");
                    //         break;
                    //     }
                    //     Ok(_) => (),
                    // },
                    DiscordThreadCommands::Update => {
                        if let Some(c) = &mut client {
                            update(&mut config, c);
                        }
                        ddrpc_log(&format!("{:?}", config))
                    }
                }
            }
        }) {
        Err(error) => return Err(error),
        Ok(_) => (),
    }

    Ok(sender_discord)
}
