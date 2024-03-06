use interprocess::local_socket::LocalSocketStream;
use std::{io::BufReader, process};

use crate::daemon::socket::write;

pub fn ipc_parser(buffer: String, socket_stream: &mut BufReader<LocalSocketStream>) -> () {
    let split_buffer: Vec<String> = buffer
        .split_whitespace()
        .map(|str| str.to_lowercase())
        .collect();
    let final_buffer: Vec<&str> = split_buffer.iter().map(|string| string.as_ref()).collect();

    match final_buffer[0] {
        "ping" => write(socket_stream, b"pong"),
        "kill" => {
            write(socket_stream, b"Killing process");
            process::exit(0);
        }
        _ => write(socket_stream, b"Unknown input"),
    }
}
