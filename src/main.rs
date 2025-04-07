use chrono::Local;
use send::DBUS;
use std::{thread::sleep, time::Duration};

mod feed;
mod parse;
mod send;

#[tokio::main]
async fn main() {
    let feeds = parse::get_feeds();
    let mut checked = Local::now().fixed_offset();
    let dbus = DBUS::new();

    loop {
        println!("Parsing all feeds...");
        let _ = parse::parse_feeds(&dbus, &feeds, &checked).await;
        checked = Local::now().fixed_offset();
        println!("Sleeping...");
        sleep(Duration::from_secs(60 * 60));
    }
}

