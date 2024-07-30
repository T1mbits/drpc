mod discord;
mod socket;

use anyhow::Context;
use common::{
    config::{get_config, Config},
    ipc::*,
    log::*,
    spotify::try_authentication_from_cache,
};
use discord::{disconnect_discord, discord_thread};
use socket::Socket;
use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        mpsc::Sender,
        Arc,
    },
    time::Duration,
};
use tokio::{spawn, sync::RwLock, time::timeout};

static RUNNING: AtomicBool = AtomicBool::new(true);

fn daemon_running() -> bool {
    RUNNING.load(Ordering::SeqCst)
}

fn kill_daemon() -> () {
    RUNNING.store(false, Ordering::SeqCst);
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    ctrlc::set_handler(|| kill_daemon()).context("Error setting up ctrlc handler")?;
    log_init(LevelFilter::Trace);
    let socket = Socket::new().await?;
    let config = Arc::new(RwLock::new(match get_config(false) {
        Err(e) => {
            warn!("{e}");
            info!("Using default config");
            Config::default()
        }
        Ok(c) => c,
    }));

    let discord_sender: Arc<RwLock<Option<Sender<()>>>> = Arc::new(RwLock::new(None));

    let spotify_client = Arc::new(RwLock::new(try_authentication_from_cache().await.unwrap()));

    while daemon_running() {
        if let Ok(result) = timeout(Duration::from_millis(100), socket.accept()).await {
            let mut stream = match result {
                Err(err) => {
                    error!("Error accepting unix socket connection: {err}");
                    continue;
                }
                Ok((stream, _)) => stream,
            };
            let config = config.clone();
            let discord_sender = discord_sender.clone();
            let spotify_client = spotify_client.clone();

            spawn(async move {
                match read(&mut stream).await.unwrap() {
                    Some(msg) => match msg {
                        IpcMessage::Activity(activity) => config.write().await.activity = activity,
                        IpcMessage::Connect(id) => discord_thread(
                            id,
                            config,
                            &mut *discord_sender.write().await,
                            spotify_client,
                        ),
                        IpcMessage::Disconnect => {
                            disconnect_discord(&mut *discord_sender.write().await)
                        }
                        IpcMessage::Kill => kill_daemon(),
                        IpcMessage::Ping => write(IpcMessage::Ping, &mut stream).await.unwrap(),
                        _ => todo!(),
                    },
                    None => error!("An empty message was received"),
                }
            });
        }
    }
    drop(socket);
    disconnect_discord(&mut *discord_sender.write().await);

    info!("Exiting...");
    Ok(())
}
