mod socket;

use anyhow::Context;
use common::{ipc::*, log::*};
use socket::Socket;
use std::{
    io::ErrorKind,
    os::unix::net::UnixStream,
    sync::atomic::{AtomicBool, Ordering},
    thread::{sleep, spawn},
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

    while daemon_running() {
        match socket.accept() {
            Err(e) if e.kind() == ErrorKind::WouldBlock => sleep(Duration::from_millis(100)),
            Err(e) => {
                error!("error accepting unix socket connection: {e}");
            }
            Ok((s, _)) => parse_message(s),
        }
    }

    drop(socket);

    info!("Exiting...");
    Ok(())
}

fn parse_message(mut stream: UnixStream) {
    trace!("parsing");
    spawn(move || {
        match read(&mut stream).unwrap() {
            IpcMessage::Incomplete => error!("Received an incomplete or blank message"),
            IpcMessage::Kill => kill_daemon(),
            IpcMessage::Ping => write(IpcMessage::Ping, &mut stream).unwrap(),
            IpcMessage::Unknown(m) => error!("Received unknown message: {m}"),
            m => error!("Unimplemented message: {m}"),
        };
    });
}
