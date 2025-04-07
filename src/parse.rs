use crate::{feed::Feed, send::DBUS};
use base64::{Engine, prelude::BASE64_STANDARD};
use chrono::{DateTime, FixedOffset};
use std::error::Error;

pub async fn parse_feeds(
    connection: &DBUS,
    feeds: &Vec<(String, Vec<u8>)>,
    last_sync: &DateTime<FixedOffset>,
) -> Result<(), Box<dyn Error>> {
    for feed in feeds {
        let feed_url = &feed.0;
        let group = &feed.1;

        let articles = Feed::new(feed_url).await?.articles;
        for article in articles {
            let time = &article.time();
            if time < last_sync {
                continue;
            }

            let title = article.title();
            let url = article.url();
            let message = &format!("{}\n{}", title, url);
            connection.send(message, group);
        }
    }

    Ok(())
}

fn arg() -> String {
    std::env::args()
        .collect::<Vec<_>>()
        .get(1)
        .unwrap_or(&String::default())
        .to_string()
}

pub fn get_feeds() -> Vec<(String, Vec<u8>)> {
    let file = arg();
    if let Ok(feeds) = std::fs::read_to_string(file) {
        return feeds
            .lines()
            .map(|line| {
                let mut parts = line.split_whitespace();
                let group = parts.next().unwrap_or_default().to_string();
                let url = parts.next().unwrap_or_default().to_string();
                let url_decoded = BASE64_STANDARD.decode(url).unwrap_or_default();
                (group, url_decoded)
            })
            .collect();
    }

    eprintln!("Couldn't read from the file");
    std::process::exit(1);
}
