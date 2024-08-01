use common::{
    config::{Config, Template},
    log::*,
    spotify::get_song_data,
};
use discord_rich_presence::{DiscordIpc, DiscordIpcClient};
use rspotify::AuthCodePkceSpotify;
use std::{
    sync::{
        mpsc::{channel, RecvTimeoutError, Sender},
        Arc,
    },
    thread::Builder,
    time::Duration,
};
use tokio::{runtime::Runtime, sync::RwLock};

fn client_init(client_id: u64) -> DiscordIpcClient {
    let mut client = DiscordIpcClient::new(&client_id.to_string())
        .expect("a new DiscordIpcClient should always be able to be created. Not quite sure why this even returns an error");

    client.connect().unwrap();

    client
}

pub async fn discord_thread(
    id: u64,
    config: Arc<RwLock<Config>>,
    sender: Arc<RwLock<Option<Sender<()>>>>,
    spotify_client: Arc<RwLock<AuthCodePkceSpotify>>,
) {
    if sender.read().await.is_some() {
        return error!("Already connected to Discord");
    }

    let (send, recv) = channel();
    *sender.write().await = Some(send);

    Builder::new()
        .name("discord".to_string())
        .spawn(move || {
            Runtime::new().unwrap().block_on(async {
                let mut client = client_init(id);

                loop {
                    let config = config.read().await;
                    let activity = config.activity.evaluate();

                    client.set_activity(activity).unwrap();

                    debug!(
                        "{}",
                        get_song_data(spotify_client.clone()).await.unwrap().name
                    );

                    match recv.recv_timeout(Duration::from_secs(1)) {
                        Err(err) => {
                            if err == RecvTimeoutError::Disconnected {
                                panic!("Channel sender dropped");
                            }
                        }
                        Ok(_) => {
                            client.close().unwrap();
                            break;
                        }
                    }
                }
            })
        })
        .unwrap();
}

pub async fn disconnect_discord(sender: Arc<RwLock<Option<Sender<()>>>>) {
    let sender_guard = sender.read().await;
    if let Some(send) = sender_guard.as_ref() {
        if let Err(_) = send.send(()) {
            error!("The Discord thread receiver hung up");
        }
        drop(sender_guard);
        *sender.write().await = None;
    }
}
