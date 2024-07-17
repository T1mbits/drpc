use crate::log::*;
use anyhow::Context;
use serde::{Deserialize, Serialize};
use serde_cbor::{from_reader, to_vec};
use std::{
    fmt::Display,
    io::{Read, Write},
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
    /// An incomplete message was sent. This typically means that no response was sent back from the daemon
    Incomplete,
    /// Kill the daemon
    Kill,
    /// Request for a ping response from the daemon.
    Ping,
    /// Represents the success of an operation of the daemon. Used for simple confirmations
    Success(bool),
    /// Unknown message
    Unknown(String),
}

impl Display for IpcMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let message = match self {
            Self::Connect(i) => format!("Connect to Discord IPC with client id: {i}"),
            Self::Disconnect => "Disconnect from Discord IPC".to_string(),
            Self::Incomplete => {
                "Incomplete message. This typically means no message was received".to_string()
            }
            Self::Kill => "Kill".to_string(),
            Self::Ping => "Ping".to_string(),
            Self::Unknown(r) => format!("Unknown: {r}"),
            r => format!("Unimplemented: {r}"),
        };

        write!(f, "{message}")
    }
}

/// Write an [`IpcMessage`] to a socket stream
pub fn write(message: IpcMessage, stream: &mut UnixStream) -> anyhow::Result<()> {
    stream
        .write(&to_vec(&message).context("failed to serialize message into CBOR")?)
        .context("failed to write message to unix socket")?;
    Ok(())
}

// TODO add edge cases for unknown messages
/// Read an [`IpcMessage`] from a socket stream.
pub fn read(stream: &mut UnixStream) -> anyhow::Result<IpcMessage> {
    let mut buf: Vec<u8> = vec![];
    stream.read_to_end(&mut buf)?;

    match from_reader(&*buf) {
        Err(e) if e.is_eof() => Ok(IpcMessage::Incomplete),
        Err(e) => Err(e.into()),
        Ok(m) => {
            trace!("{:#?}", m);
            Ok(m)
        }
    }
}
