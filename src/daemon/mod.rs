use chrono::offset::Local;
use fork::{daemon, Fork};
use std::{io::prelude::*, process::Command};

pub fn start_daemon() {
    if let Ok(Fork::Child) = daemon(false, false) {
        loop {
            let mut file = std::fs::OpenOptions::new()
                .write(true)
                .append(true)
                .open("/home/Timbits/.config/drpc/daemon-log.txt")
                .unwrap();
            let time = Local::now();
            if let Err(e) = writeln!(file, "{time}") {
                panic!("Error: {}", e);
            }
            std::thread::sleep(std::time::Duration::from_millis(1000));
        }
    }
}

pub fn kill_daemon() {
    Command::new("pkill")
        .arg("drpc")
        .output()
        .expect("Oh nooooo");
}
