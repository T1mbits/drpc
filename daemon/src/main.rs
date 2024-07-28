mod discord;
mod socket;

use anyhow::Context;
use common::{
    config::{get_config, Config},
    ipc::*,
    log::*,
};
use discord::{disconnect_discord, discord_thread, update_activity, DiscordChannelMessage};
use socket::Socket;
use std::{
    io::ErrorKind,
    sync::{
        atomic::{AtomicBool, Ordering},
        mpsc::Sender,
    },
    thread::sleep,
    time::Duration,
};

static RUNNING: AtomicBool = AtomicBool::new(true);

fn daemon_running() -> bool {
    RUNNING.load(Ordering::SeqCst)
}

fn kill_daemon() -> () {
    RUNNING.store(false, Ordering::SeqCst);
}

fn main() -> anyhow::Result<()> {
    ctrlc::set_handler(|| kill_daemon()).context("Error setting up ctrlc handler")?;
    log_init(LevelFilter::Trace);

    let socket = Socket::new()?;
    let mut config = match get_config(false) {
        Err(e) => {
            warn!("{e}");
            info!("Using default config");
            Config::default()
        }
        Ok(c) => c,
    };
    let mut discord_sender: Option<Sender<DiscordChannelMessage>> = None;

    while daemon_running() {
        match socket.accept() {
            Err(err) if err.kind() == ErrorKind::WouldBlock => sleep(Duration::from_millis(100)),
            Err(err) => error!("Error accepting unix socket connection: {err}"),
            Ok((mut stream, _)) => match read(&mut stream).unwrap() {
                Some(msg) => match msg {
                    IpcMessage::Activity(activity) => {
                        config.activity = activity;
                        update_activity(&config.activity, &discord_sender);
                    }
                    IpcMessage::Connect(id) => discord_thread(id, &config, &mut discord_sender),
                    IpcMessage::Disconnect => disconnect_discord(&mut discord_sender),
                    IpcMessage::Kill => kill_daemon(),
                    IpcMessage::Ping => write(IpcMessage::Ping, &mut stream)?,
                    _ => todo!(),
                },
                None => error!("An empty message was received"),
            },
        }
    }

    drop(socket);
    disconnect_discord(&mut discord_sender);

    info!("Exiting...");
    Ok(())
}
