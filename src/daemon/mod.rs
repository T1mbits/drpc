pub mod socket;

use fork::{daemon, Fork};
use interprocess::local_socket::LocalSocketListener;
use std::{
    io::{self, ErrorKind},
    process,
    sync::mpsc::{channel, Receiver, Sender},
};

use crate::{
    config::structure::DConfig,
    daemon::socket::*,
    discord::{discord_thread, DiscordThreadCommands},
    logging::ddrpc_log,
    parser::ipc::ipc_parser,
};

pub struct ChannelCommunications<'cc> {
    pub discord: Sender<DiscordThreadCommands>,
    pub main: &'cc Receiver<Vec<u8>>,
}

pub fn start_daemon() {
    let socket_input_name = input_socket_path();

    let listener: LocalSocketListener = match create_listener(socket_input_name) {
        Err(error) if error.kind() == io::ErrorKind::AddrInUse => {
            ddrpc_log(&format!("Socket in use: {error}"));
            eprintln!("Socket is already bound to another listener. Use `ddrpc ping` to check if another daemon is active.");
            process::exit(1);
        }
        Err(error) => {
            ddrpc_log(&format!("Error while binding to socket: {error}"));
            eprintln!("Error while binding to socket: {error}");
            process::exit(1);
        }
        Ok(socket_listener) => socket_listener,
    };
    println!("Created and bound socket listener to {socket_input_name}");
    ddrpc_log(&format!(
        "Created and bound socket listener to {socket_input_name}"
    ));

    println!("Forking into daemon...");
    ddrpc_log("Forking into daemon...");
    if let Ok(Fork::Child) = daemon(false, false) {
        let (sender_main, receiver_main) = channel();
        ddrpc_log("Forked into daemon");

        let discord_sender = match discord_thread(DConfig::default().discord, sender_main.clone()) {
            Err(error) => {
                ddrpc_log(&format!("Error while creating Discord RPC thread: {error}"));
                process::exit(1);
            }
            Ok(sender) => sender,
        };

        ddrpc_log("Created Discord RPC connection thread");

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
            ipc_parser(
                response.buffer,
                &mut response.buf_reader_socket_stream,
                &ChannelCommunications {
                    discord: discord_sender.clone(),
                    main: &receiver_main,
                },
            );
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
    println!("Successfully killed daemon");
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
                if error.kind() == ErrorKind::ConnectionRefused {
                    eprintln!("The daemon may not be active. Try using \"ddrpc start\" to start the daemon.");
                }
                process::exit(1);
            }
            Ok(buffer) => buffer,
        }
    );
}

pub fn get_discord() {
    print!(
        "{}",
        match exchange(b"discord get", input_socket_path()) {
            Err(error) => {
                eprintln!(
                    "An error occurred while trying to exchange messages over the socket: {}",
                    error
                );
                if error.kind() == ErrorKind::ConnectionRefused {
                    eprintln!("The daemon may not be active. Try using \"ddrpc start\" to start the daemon.");
                }
                process::exit(1);
            }
            Ok(buffer) => buffer,
        }
    );
}
