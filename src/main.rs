use base64::prelude::*;
use chrono::{DateTime, Utc};

use atom_syndication::Feed;
use core::panic;
use dbus::blocking::LocalConnection;
use rss::Channel;
use std::{env::args, error::Error, fs, thread::sleep, time::Duration};

async fn parse_feeds(feeds: &str, checked: &DateTime<Utc>) -> Result<Vec<String>, Box<dyn Error>> {
    let mut messages: Vec<String> = Vec::new();

    for feed in feeds.lines() {
        let parts: Vec<&str> = feed.split_whitespace().collect();
        let format = parts[0];
        let url = parts[1];
        let content = &reqwest::get(url).await?.bytes().await?[..];

        match format {
            "atom" => {
                let feed = Feed::read_from(content)?;
                for entry in feed.entries {
                    let time = entry.published.unwrap();
                    if &time <= checked {
                        continue;
                    }

                    let title = entry.title.value;
                    let link = entry.links[0].href.clone();
                    let message = format!("{}\n{}", title, link);

                    messages.push(message);
                }
            }
            "rss" => {
                let feed = Channel::read_from(content)?;
                for entry in feed.items {
                    let time = DateTime::parse_from_rfc2822(&entry.pub_date.unwrap())?;
                    if &time <= checked {
                        continue;
                    }

                    let title = entry.title.unwrap();
                    let link = entry.link.unwrap();
                    let message = format!("{}\n{}", title, link);

                    messages.push(message);
                }
            }
            _ => panic!(),
        }
    }

    Ok(messages)
}

#[tokio::main]
async fn main() {
    let group = get_group();
    let feeds = get_feeds();
    let mut checked = Utc::now();

    loop {
        parse_feeds(&feeds, &checked)
            .await
            .unwrap()
            .iter()
            .for_each(|message| {
                let _ = send(&message, &group);
            });

        checked = Utc::now();
        sleep(Duration::from_secs(60 * 60));
    }
}

fn get_group() -> Vec<u8> {
    let args: Vec<String> = args().collect();
    let id = &args[1];
    BASE64_STANDARD.decode(id).unwrap()
}

fn get_feeds() -> String {
    let args: Vec<String> = args().collect();
    let file = &args[2];
    fs::read_to_string(file).unwrap()
}

fn send(message: &str, group: &Vec<u8>) -> Result<(), Box<dyn Error>> {
    let con = LocalConnection::new_session()?;
    let proxy = con.with_proxy(
        "org.asamk.Signal",
        "/org/asamk/Signal",
        Duration::from_secs(5),
    );
    let _: (Vec<String>,) = proxy.method_call(
        "org.asamk.Signal",
        "sendGroupMessage",
        (message, Vec::<&str>::new(), group),
    )?;
    Ok(())
}
