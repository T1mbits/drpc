use crate::config::{ActivityTemplate, ProcessesConfig};
use anyhow::{anyhow, Context};
use ciborium::{de::Error, from_reader, into_writer};
use serde::{Deserialize, Serialize};
use std::io::ErrorKind;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::UnixStream,
};

pub const SOCKET_FILE: &str = "/tmp/ddrpc.socket";

/// Messages sent or received via IPC
#[derive(Debug, Deserialize, Serialize)]
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
}

/// Write an [`IpcMessage`] to a socket stream
pub async fn write(message: IpcMessage, stream: &mut UnixStream) -> anyhow::Result<()> {
    let mut buffer = Vec::new();
    into_writer(&message, &mut buffer).context("failed to serialize message into CBOR")?;
    stream
        .write(&buffer)
        .await
        .context("failed to write message to unix socket")?;
    Ok(())
}

/// Read an [`IpcMessage`] from a socket stream. Will return `Ok(None)` when a blank buffer is read
pub async fn read(stream: &mut UnixStream) -> anyhow::Result<Option<IpcMessage>> {
    let mut buf: Vec<u8> = vec![];
    stream.read_to_end(&mut buf).await?;

    match from_reader(&*buf) {
        Err(err) => match err {
            Error::Io(err) if err.kind() == ErrorKind::UnexpectedEof => {
                if buf.is_empty() {
                    return Ok(None);
                }
                Err(err.into())
            }
            err => Err(err.into()),
        },
        Ok(msg) => Ok(msg),
    }
}

/// Write a message to the socket and automatically shut down the writing stream and read any message sent back.
///
/// # Errors
/// Returns an error if the socket fails to connect, if an error occurs during [`write`], [`UnixStream`] write shutdown, or [`read`]
pub async fn message(message: IpcMessage) -> anyhow::Result<Option<IpcMessage>> {
    match UnixStream::connect(SOCKET_FILE).await {
        Err(err) if err.kind() == ErrorKind::NotFound => Err(anyhow!("No daemon was found")),
        Err(err) if err.kind() == ErrorKind::ConnectionRefused => {
            Err(anyhow!("A socket was found, but no daemon responded"))
        }
        Err(err) => Err(err.into()),
        Ok(mut stream) => {
            write(message, &mut stream).await?;
            stream.shutdown().await?;

            read(&mut stream).await
        }
    }
}
