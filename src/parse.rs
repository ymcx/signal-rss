use crate::{connection::SignalConnection, feed::Feed};
use base64::{Engine, prelude::BASE64_STANDARD};
use chrono::{DateTime, FixedOffset, Local};
use std::error::Error;

pub async fn parse_feeds(
    connection: &SignalConnection,
    feeds: &Vec<(Vec<u8>, String)>,
    last_sync: &DateTime<FixedOffset>,
) -> Result<(), Box<dyn Error>> {
    for feed in feeds {
        let group = &feed.0;
        let feed_url = &feed.1;

        let articles = Feed::new(feed_url).await?.articles;
        for article in articles {
            let time = &article.time();
            if time < last_sync {
                continue;
            }

            let title = article.title();
            let url = article.url();
            let message = &format!("{}\n{}", title, url);
            connection.send(message, group)?;
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

pub fn time() -> DateTime<FixedOffset> {
    Local::now().fixed_offset()
}

pub fn get_feeds() -> Vec<(Vec<u8>, String)> {
    let file = arg();
    if let Ok(feeds) = std::fs::read_to_string(file) {
        return feeds
            .lines()
            .map(|line| {
                let mut parts = line.split_whitespace();
                let group = parts.next().unwrap_or_default().to_string();
                let group_decoded = BASE64_STANDARD.decode(group).unwrap_or_default();
                let url = parts.next().unwrap_or_default().to_string();
                (group_decoded, url)
            })
            .collect();
    }

    eprintln!("Couldn't read from the file");
    std::process::exit(1);
}
