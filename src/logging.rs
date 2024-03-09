use chrono::Local;
use std::io::prelude::*;

pub fn ddrpc_log(data: &str) {
    let mut file = std::fs::OpenOptions::new()
        .write(true)
        .append(true)
        .open("/home/Timbits/.config/ddrpc/daemon-log.txt")
        .unwrap();
    let time = Local::now();

    let mut data = data.to_owned();
    data.retain(|char| char != '\n');

    if let Err(e) = writeln!(file, "{time} > {data}") {
        panic!("Error: {}", e);
    }
}

#[allow(dead_code)]
fn daemon_log(data: String) {
    let mut file = std::fs::OpenOptions::new()
        .write(true)
        .append(true)
        .open("/home/Timbits/.config/ddrpc/daemon-log.txt")
        .unwrap();
    let time = Local::now();

    let mut data = data.to_owned();
    data.retain(|char| char != '\n');

    if let Err(e) = writeln!(file, "[Daemon] {time} > {data}") {
        panic!("Error: {}", e);
    }
}

#[allow(dead_code)]
fn client_log(data: String) {
    let mut file = std::fs::OpenOptions::new()
        .write(true)
        .append(true)
        .open("/home/Timbits/.config/ddrpc/daemon-log.txt")
        .unwrap();
    let time = Local::now();

    let mut data = data.to_owned();
    data.retain(|char| char != '\n');

    if let Err(e) = writeln!(file, "[Client] {time} > {data}") {
        panic!("Error: {}", e);
    }
}
