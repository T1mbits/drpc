use common::{
    config::{ActivityTemplate, Config, Template},
    log::*,
};
use discord_rich_presence::{DiscordIpc, DiscordIpcClient};
use std::{
    sync::mpsc::{channel, RecvTimeoutError, Sender},
    thread::Builder,
    time::Duration,
};

pub enum DiscordChannelMessage {
    Update(ActivityTemplate),
    Disconnect,
}

fn client_init(client_id: u64) -> DiscordIpcClient {
    let mut client = DiscordIpcClient::new(&client_id.to_string())
        .expect("a new DiscordIpcClient should always be able to be created. Not quite sure why this even returns an error");

    client.connect().unwrap();

    client
}

// TODO change parameter to be full activity data
pub fn discord_thread(
    id: u64,
    config: &Config,
    sender: &mut Option<Sender<DiscordChannelMessage>>,
) {
    if let Some(_) = sender {
        return error!("A Discord connection already exists.");
    }

    let (send, recv) = channel();
    *sender = Some(send);
    let config = config.clone();

    Builder::new()
        .name("discord".to_string())
        .spawn(move || {
            let mut activity = config.activity.evaluate();

            let mut client = client_init(id);

            client.set_activity(activity).unwrap();

            loop {
                match recv.recv_timeout(Duration::from_secs(1)) {
                    Err(err) => {
                        if err == RecvTimeoutError::Disconnected {
                            panic!("Channel sender dropped");
                        }
                    }
                    Ok(msg) => match msg {
                        DiscordChannelMessage::Disconnect => {
                            client.close().unwrap();
                            break;
                        }
                        DiscordChannelMessage::Update(template) => {
                            activity = template.evaluate();
                            client.set_activity(activity).unwrap();
                        }
                    },
                }
            }
        })
        .unwrap();
}

pub fn update_activity(
    activity: &ActivityTemplate,
    sender: &Option<Sender<DiscordChannelMessage>>,
) {
    if let Some(sender) = sender {
        if let Err(_) = sender.send(DiscordChannelMessage::Update(activity.clone())) {
            error!("The Discord thread receiver hung up")
        }
    }
}

pub fn disconnect_discord(sender: &mut Option<Sender<DiscordChannelMessage>>) {
    if let Some(send) = sender {
        if let Err(_) = send.send(DiscordChannelMessage::Disconnect) {
            error!("The Discord thread receiver hung up");
        }
        *sender = None;
    }
}
