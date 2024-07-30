use anyhow::{anyhow, Context};
use common::{ipc::*, log::*};
use std::{
    fs,
    io::{self, ErrorKind},
    path::Path,
};
use tokio::{
    io::AsyncWriteExt,
    net::{unix::SocketAddr, UnixListener, UnixStream},
};

/// A wrapper for a Unix domain socket that removes its socket file when dropped
pub struct Socket {
    socket: UnixListener,
}

impl Drop for Socket {
    fn drop(&mut self) {
        match fs::remove_file(SOCKET_FILE) {
            Err(e) => error!("Failed to remove socket file: {e}"),
            Ok(_) => info!("Removed socket file"),
        }
    }
}

impl Socket {
    pub async fn new() -> anyhow::Result<Self> {
        if Path::new(SOCKET_FILE).exists() {
            trace!("found existing socket file, attempting to connect");

            match UnixStream::connect(SOCKET_FILE).await {
                Err(err) if err.kind() == ErrorKind::ConnectionRefused => {
                    fs::remove_file(SOCKET_FILE)
                        .context(format!("Failed to remove old socket file: {err}"))?;
                    warn!("Removed old socket file");
                }
                Err(err) => return Err(anyhow!("Failed to access old socket file: {err}")),
                Ok(mut stream) => {
                    write(IpcMessage::Ping, &mut stream).await?;
                    stream.shutdown().await?;

                    match read(&mut stream).await? {
                        Some(msg) => match msg {
                            IpcMessage::Ping => {
                                return Err(anyhow!(
                                    "There is already an instance of ddrpc-daemon running"
                                ))
                            }
                            _ => {
                                return Err(anyhow!(
                                    "Another program sent an unexpected message over the socket"
                                ))
                            }
                        },
                        None => return Err(anyhow!("No message was received from the daemon")),
                    }
                }
            }
        }

        let socket = UnixListener::bind(SOCKET_FILE).context("Could not create unix socket")?;

        Ok(Self { socket })
    }

    pub async fn accept(&self) -> io::Result<(UnixStream, SocketAddr)> {
        Ok(self.socket.accept().await?)
    }
}
