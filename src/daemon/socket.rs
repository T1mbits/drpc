use interprocess::local_socket::{LocalSocketListener, LocalSocketStream, NameTypeSupport};
use std::{
    io::{self, prelude::*, BufReader},
    process::Command,
};

use super::super::logging::ddrpc_log;

/// Logs any socket connection errors.<br>
/// Will return `None` if an error is encountered, otherwise it will return `Some(LocalSocketStream)`
pub fn handle_connection_error(
    connection: io::Result<LocalSocketStream>,
) -> Option<LocalSocketStream> {
    match connection {
        Err(error) => {
            ddrpc_log(&format!("Incoming connection failed: {error}"));
            None
        }
        Ok(socket_stream) => Some(socket_stream),
    }
}

/// Returns a string slice for the domain socket input location.<br>
/// A `todo!` macro is present because I have no idea what that branch does.
pub fn input_socket_path() -> &'static str {
    use NameTypeSupport::*;
    match NameTypeSupport::query() {
        OnlyPaths => todo!(),
        OnlyNamespaced | Both => "@/Timbits/ddrpc_in.sock",
    }
}

/// Returns a string slice for the domain socket output location.<br>
/// A `todo!` macro is present because I have no idea what that branch does.
pub fn output_socket_path() -> &'static str {
    use NameTypeSupport::*;
    match NameTypeSupport::query() {
        OnlyPaths => todo!(),
        OnlyNamespaced | Both => "@/Timbits/ddrpc_out.sock",
    }
}

/// Creates a `LocalSocketListener` while logging any errors during creation.<br><br>
/// If an `io:ErrorKind::AddrInUse` error is encountered, will attempt to kill any other ddrpc processes **which was pretty dumb as it kills itself**<br>
/// ~and will try to connect again, panicking if another error is encountered.~ Will panic on any other error scenario.<br><br>
/// definitely need to rewrite this
pub fn create_listener(socket_name: &str) -> LocalSocketListener {
    match LocalSocketListener::bind(socket_name) {
        Err(error) if error.kind() == io::ErrorKind::AddrInUse => {
            ddrpc_log(&format!("Error while binding to socket: {error}"));
            ddrpc_log("Killing any existing ddrpc process(es)");
            match Command::new("pkill").arg("ddrpc").output() {
                Err(error) => {
                    ddrpc_log(&format!("Error while killing ddrpc process(es): {error}"));
                    panic!()
                }
                Ok(_) => ddrpc_log("Successfully killed process(es)"),
            };
            match LocalSocketListener::bind(socket_name) {
                Err(error) => {
                    ddrpc_log(&format!(
                        "Error while reattempting to bind to socket: {error}"
                    ));
                    panic!()
                }
                Ok(listener) => listener,
            }
        }
        Err(error) => {
            ddrpc_log(&format!("Error while binding to socket: {error}"));
            panic!()
        }
        Ok(listener) => listener,
    }
}

/// Creates a connection to the given socket. Will exit with code 1 if an error is encountered.
pub fn create_connection(socket_name: &str) -> Result<LocalSocketStream, io::Error> {
    match LocalSocketStream::connect(socket_name) {
        Err(error) => {
            ddrpc_log(&format!(
                "An error occurred while connecting to {socket_name}: {error}"
            ));
            Err(error)
        }
        Ok(socket_stream) => Ok(socket_stream),
    }
}

/// Datatype containing the socket stream read from and the buffer value read from it
pub struct SocketConnectionResponse {
    pub buffer: String,
    pub buf_reader_socket_stream: BufReader<LocalSocketStream>,
}

/// Processes any socket stream connections and returns a `SocketConnectionResponse`
pub fn listener_receive(
    connection: LocalSocketStream,
) -> Result<SocketConnectionResponse, io::Error> {
    let mut connection = BufReader::new(connection);
    let buffer: &mut String = &mut String::new();

    match connection.read_line(buffer) {
        Err(error) => Err(error),
        Ok(_) => Ok(SocketConnectionResponse {
            buffer: buffer.to_owned(),
            buf_reader_socket_stream: connection,
        }),
    }
}

/// Writes the given bytes to the given `BufReader<LocalSocketStream>`.<br>
/// Automatically logs any errors.
pub fn write(socket_stream: &mut BufReader<LocalSocketStream>, buffer: &[u8]) -> () {
    let mut buffer = buffer.to_vec();
    buffer.push(b'\n');
    match socket_stream.get_mut().write_all(&buffer) {
        Err(error) => ddrpc_log(&format!("Error while writing connection: {error}")),
        Ok(_) => ddrpc_log(&format!(
            "Successfully wrote \"{}\" to buffer",
            std::str::from_utf8(&buffer).unwrap()
        )),
    };
}

/// Send a buffer to the given socket path
pub fn send(buffer: &[u8], socket_path: &str) -> Result<BufReader<LocalSocketStream>, io::Error> {
    let socket_stream: LocalSocketStream = match create_connection(socket_path) {
        Err(error) => return Err(error),
        Ok(socket_stream) => socket_stream,
    };
    let mut buffered_socket_stream: BufReader<LocalSocketStream> = BufReader::new(socket_stream);
    write(&mut buffered_socket_stream, buffer);
    Ok(buffered_socket_stream)
}

/// Read a buffer sent from the given socket stream.<br>
/// Will block the thread until a newline character (`0xA` byte) is given
pub fn receive(socket_stream: BufReader<LocalSocketStream>) -> Result<String, io::Error> {
    let mut buffer = String::new();
    let mut socket_stream = socket_stream;
    match socket_stream.read_line(&mut buffer) {
        Err(error) => Err(error),
        Ok(_) => Ok(buffer),
    }
}

/// Exchange messages with the given socket
pub fn exchange(buffer: &[u8], socket: &str) -> Result<String, io::Error> {
    receive(match send(buffer, socket) {
        Err(error) => return Err(error),
        Ok(socket_stream) => socket_stream,
    })
}
