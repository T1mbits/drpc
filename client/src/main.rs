use common::ipc::*;
use std::{net::Shutdown, os::unix::net::UnixStream};

fn main() -> anyhow::Result<()> {
    let mut stream = UnixStream::connect(SOCKET_FILE)?;
    write(IpcMessage::Ping, &mut stream)?;
    stream.shutdown(Shutdown::Write)?;
    println!("Server response: {}", read(&mut stream)?);

    Ok(())
}
