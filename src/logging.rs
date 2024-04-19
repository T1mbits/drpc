use chrono::Local;
use slog::{self, o, Drain, Logger};
use slog_async;
use slog_term;
use std::{fs::OpenOptions, io::prelude::*};

use crate::config::dir_path;

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

pub fn slog_init() -> Logger {
    let output_path = dir_path() + "ddrpc.log";
    let file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(output_path)
        .unwrap();

    let decorator = slog_term::PlainSyncDecorator::new(file);
    let drain = slog_term::FullFormat::new(decorator).build().fuse();
    let drain = slog_async::Async::new(drain).build().fuse();

    slog::Logger::root(drain, o!())
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
