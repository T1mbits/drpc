use crate::config::{ActivityTemplate, ProcessesConfig, SpotifyConfig};
use anyhow::{anyhow, Context};
use serde::{Deserialize, Serialize};
use serde_cbor::{from_reader, to_vec};
use std::{
    io::{ErrorKind, Read, Write},
    net::Shutdown,
    os::unix::net::UnixStream,
};

pub const SOCKET_FILE: &str = "/tmp/ddrpc.socket";

/// Messages sent or received via IPC
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub enum IpcMessage {
    /// Connect to the Discord IPC with given Discord client ID
    Connect(u64),
    /// Disconnect from the Discord IPC
    Disconnect,
    /// Kill message for the daemon
    Kill,
    /// Request for a ping response from the daemon.
    Ping,
    /// A message containing an ActivityTemplate
    Activity(ActivityTemplate),
    /// A message containing a ProcessesConfig
    Processes(ProcessesConfig),
    /// A message containing a SpotifyConfig
    Spotify(SpotifyConfig),
}

/// Write an [`IpcMessage`] to a socket stream
pub fn write(message: IpcMessage, stream: &mut UnixStream) -> anyhow::Result<()> {
    stream
        .write(&to_vec(&message).context("failed to serialize message into CBOR")?)
        .context("failed to write message to unix socket")?;
    Ok(())
}

// TODO add error returns for unknown messages
/// Read an [`IpcMessage`] from a socket stream.
pub fn read(stream: &mut UnixStream) -> anyhow::Result<Option<IpcMessage>> {
    let mut buf: Vec<u8> = vec![];
    stream.read_to_end(&mut buf)?;

    match from_reader(&*buf) {
        Err(e) if e.is_eof() => {
            if buf.is_empty() {
                return Ok(None);
            }
            Err(anyhow!("An incomplete message was received: {:?}", buf))
        }
        Err(e) => Err(e.into()),
        Ok(m) => Ok(m),
    }
}

/// Write a message to the socket and automatically shut down the writing stream and read any message sent back.
///
/// # Errors
/// Returns an error if the socket fails to connect, if an error occurs during [`write`], [`UnixStream`] write shutdown, or [`read`]
pub fn message(message: IpcMessage) -> anyhow::Result<Option<IpcMessage>> {
    match UnixStream::connect(SOCKET_FILE) {
        Err(e) if e.kind() == ErrorKind::NotFound => Err(anyhow!("No daemon was found")),
        Err(e) if e.kind() == ErrorKind::ConnectionRefused => {
            Err(anyhow!("A socket was found, but no daemon responded"))
        }
        Err(e) => Err(e.into()),
        Ok(mut s) => {
            write(message, &mut s)?;
            s.shutdown(Shutdown::Write)?;

            read(&mut s)
        }
    }
}
