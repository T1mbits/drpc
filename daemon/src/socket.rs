use anyhow::{anyhow, Context};
use common::{ipc::*, log::*};
use std::{
    fs,
    io::{self, ErrorKind},
    net::Shutdown,
    os::unix::net::{SocketAddr, UnixListener, UnixStream},
    path::Path,
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
    pub fn new() -> anyhow::Result<Self> {
        if Path::new(SOCKET_FILE).exists() {
            trace!("found existing socket file, attempting to connect");

            match UnixStream::connect(SOCKET_FILE) {
                Err(e) if e.kind() == ErrorKind::ConnectionRefused => {
                    fs::remove_file(SOCKET_FILE)
                        .context(format!("Failed to remove old socket file: {e}"))?;
                    warn!("Removed old socket file");
                }
                Err(e) => return Err(anyhow!("Failed to access old socket file: {e}")),
                Ok(mut s) => {
                    write(IpcMessage::Ping, &mut s)?;
                    s.shutdown(Shutdown::Write)?;
                    match read(&mut s)? {
                        IpcMessage::Ping => {
                            return Err(anyhow!(
                                "There is already an instance of ddrpc-daemon running"
                            ))
                        }
                        IpcMessage::Incomplete => {
                            return Err(anyhow!(
                                "No message or an incomplete message was sent over the socket"
                            ))
                        }
                        _ => {
                            return Err(anyhow!(
                                "Another program sent an unexpected message over the socket"
                            ))
                        }
                    }
                }
            }
        }

        let socket = UnixListener::bind(SOCKET_FILE).context("Could not create unix socket")?;
        socket.set_nonblocking(true)?;

        Ok(Self { socket })
    }

    pub fn accept(&self) -> io::Result<(UnixStream, SocketAddr)> {
        Ok(self.socket.accept()?)
    }
}
