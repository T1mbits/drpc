use interprocess::local_socket::{LocalSocketListener, LocalSocketStream, NameTypeSupport};
use std::io::{self, prelude::*, BufReader};

use crate::logging::ddrpc_log;

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

/// Returns a string slice for the domain socket location.<br>
/// A `todo!` macro is present because I have no idea what those branches do.
pub fn socket_path() -> &'static str {
    use NameTypeSupport::*;
    match NameTypeSupport::query() {
        Both => "@/Timbits/ddrpc.sock\n",
        OnlyNamespaced => todo!(),
        OnlyPaths => todo!(),
    }
}

/// Returns a string slice for the domain socket output location.<br>
/// A `todo!` macro is present because I have no idea what that branch does.
pub fn output_socket_path() -> &'static str {
    use NameTypeSupport::*;
    match NameTypeSupport::query() {
        Both => "@/Timbits/ddrpc_out.sock",
        OnlyNamespaced => todo!(),
        OnlyPaths => todo!(),
    }
}

/// Creates a `LocalSocketListener`
pub fn create_listener(socket_name: &str) -> Result<LocalSocketListener, io::Error> {
    match LocalSocketListener::bind(socket_name) {
        Err(error) => Err(error),
        Ok(listener) => Ok(listener),
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
    match socket_stream.read_to_string(&mut buffer) {
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
