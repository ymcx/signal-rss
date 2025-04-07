use chrono::Local;
use connection::SignalConnection;
use std::{thread::sleep, time::Duration};

mod connection;
mod feed;
mod parse;

#[tokio::main]
async fn main() {
    let feeds = parse::get_feeds();
    let mut checked = Local::now().fixed_offset();
    let dbus = SignalConnection::new().unwrap();

    loop {
        println!("Parsing all feeds...");
        if let Err(e) = parse::parse_feeds(&dbus, &feeds, &checked).await {
            println!("{}", e);
        }
        checked = Local::now().fixed_offset();
        println!("Sleeping...");
        sleep(Duration::from_secs(60 * 60));
    }
}
