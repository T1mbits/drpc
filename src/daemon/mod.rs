pub mod socket;

use fork::{daemon, Fork};
use interprocess::local_socket::LocalSocketListener;
use std::process;

use super::logging::*;
use crate::parser::ipc::ipc_parser;
use socket::*;

pub fn start_daemon() {
    let socket_input_name = input_socket_path();

    let listener: LocalSocketListener = create_listener(socket_input_name);
    ddrpc_log(&format!(
        "Created and bound listener to {socket_input_name}"
    ));

    ddrpc_log("Forking into daemon...");
    if let Ok(Fork::Child) = daemon(false, false) {
        ddrpc_log("Forked into daemon");

        // judging off of tests this for loop keeps the daemon active, although it does block it while waiting for
        // the tests
        for connection in listener.incoming().filter_map(handle_connection_error) {
            let mut response = match listener_receive(connection) {
                Err(error) => {
                    ddrpc_log(&format!(
                        "An error occurred while trying to receive the connection: {error}"
                    ));
                    continue;
                }
                Ok(response) => response,
            };
            ipc_parser(response.buffer, &mut response.buf_reader_socket_stream);
        }
    }
}

pub fn kill_daemon() {
    match exchange(b"kill", input_socket_path()) {
        Err(error) => {
            eprintln!(
                "An error occurred while trying to exchange messages over the socket: {}",
                error
            );
            process::exit(1);
        }
        Ok(buffer) => buffer,
    };
    print!("Successfully killed daemon");
}

pub fn ping_daemon() {
    print!(
        "{}",
        match exchange(b"ping", input_socket_path()) {
            Err(error) => {
                eprintln!(
                    "An error occurred while trying to exchange messages over the socket: {}",
                    error
                );
                process::exit(1);
            }
            Ok(buffer) => buffer,
        }
    );
}
