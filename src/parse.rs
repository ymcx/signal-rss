use crate::{feed::Feed, send::DBUS};
use chrono::{DateTime, Utc};

async fn parse_feeds2(
    dbus: &DBUS,
    group: &Vec<u8>,
    url: &str,
    checked: &DateTime<Utc>,
) {
    let feed = Feed::new(url).await;

    for entry in feed.unwrap().articles {
        let time = entry.time();
        if &time < checked {
            // continue;
        }

        let title = entry.title();
        let link = entry.url();
        let message = format!("{}\n{}", title, link);

        dbus.send(&message, group);
    }
}

pub async fn parse_feeds(dbus: &DBUS, group: &Vec<u8>, feeds: &str, checked: &DateTime<Utc>) {
    for feed in feeds.lines() {
        parse_feeds2(&dbus, &group, &feed, &checked).await;
    }
}
