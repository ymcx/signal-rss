use base64::prelude::*;
use chrono::Utc;
use send::DBUS;
use std::{env::args, fs, thread::sleep, time::Duration};

mod feed;
mod parse;
mod send;

#[tokio::main]
async fn main() {
    let group = get_group();
    let feeds = get_feeds();
    let mut checked = Utc::now();
    let dbus = DBUS::new();

    loop {
        println!("Parsing all feeds...");
        parse::parse_feeds(&dbus, &group, &feeds, &checked).await;
        checked = Utc::now();
        println!("Done!");
        sleep(Duration::from_secs(60 * 60));
    }
}

fn get_group() -> Vec<u8> {
    let args: Vec<String> = args().collect();
    let id = &args[1];
    if let Ok(group) = BASE64_STANDARD.decode(id) {
        return group;
    }

    panic!("Couldn't parse the group id");
}

fn get_feeds() -> String {
    let args: Vec<String> = args().collect();
    let file = &args[2];
    if let Ok(feeds) = fs::read_to_string(file) {
        return feeds;
    }

    panic!("Couldn't read from the feeds file");
}
