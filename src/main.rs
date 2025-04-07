use connection::SignalConnection;
use std::time::Duration;

mod connection;
mod feed;
mod parse;

#[tokio::main]
async fn main() {
    let feeds = parse::get_feeds();
    let connection = SignalConnection::new();
    let mut last_sync = parse::time();

    loop {
        let _ = parse::parse_feeds(&connection, &feeds, &last_sync)
            .await
            .map_err(|e| eprintln!("{e}"));

        last_sync = parse::time();
        tokio::time::sleep(Duration::from_secs(60 * 60)).await;
    }
}
