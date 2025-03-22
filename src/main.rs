use base64::prelude::*;
use chrono::{DateTime, Utc};

use atom_syndication::Feed;
use core::panic;
use dbus::blocking::LocalConnection;
use rss::Channel;
use std::{env::args, error::Error, fs, thread::sleep, time::Duration};

fn modify_url(url: &str, description: &str) -> String {
    if ["www.memeorandum.com", "www.techmeme.com"]
        .iter()
        .any(|&i| url.contains(i))
    {
        let parts: Vec<&str> = description.split('"').collect();
        return parts[1].to_string();
    }

    url.replace("www.youtube.com", "yewtu.be")
}

async fn parse_feeds(feeds: &str, checked: &DateTime<Utc>) -> Result<Vec<String>, Box<dyn Error>> {
    let mut messages: Vec<String> = Vec::new();

    for feed in feeds.lines() {
        let parts: Vec<&str> = feed.split_whitespace().collect();
        let format = parts[0];
        let url = parts[1];
        let content = &reqwest::get(url).await?.bytes().await?[..];

        match format {
            "atom" => {
                let feed = Feed::read_from(content);
                if let Err(_) = feed {
                    continue;
                }

                for entry in feed.unwrap().entries {
                    let time = entry.published.unwrap_or_default();
                    if &time <= checked {
                        continue;
                    }

                    let title = entry.title.value;
                    let description = entry.summary.unwrap_or_default().value;
                    let link = modify_url(&entry.links[0].href, &description);
                    let message = format!("{}\n{}", title, link);

                    messages.push(message);
                }
            }
            "rss" => {
                let feed = Channel::read_from(content);
                if let Err(_) = feed {
                    continue;
                }

                for entry in feed.unwrap().items {
                    let time = DateTime::parse_from_rfc2822(&entry.pub_date.unwrap_or_default())
                        .unwrap_or_default();
                    if &time <= checked {
                        continue;
                    }

                    let title = entry.title.unwrap_or_default();
                    let description = entry.description.unwrap_or_default();
                    let link = modify_url(&entry.link.unwrap_or_default(), &description);
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
