use atom_syndication::Entry;
use chrono::{DateTime, FixedOffset};
use quick_xml::{Reader, events::Event};
use reqwest::Url;
use rss::{Channel, Item};
use std::error::Error;

pub struct Feed {
    pub articles: Vec<Article>,
}

pub enum Article {
    Atom(Entry),
    Rss(Item),
}

impl Feed {
    fn is_atom(content: &[u8]) -> Result<bool, Box<dyn Error>> {
        let content = std::str::from_utf8(content)?;
        let mut reader = Reader::from_str(content);

        loop {
            match reader.read_event() {
                Ok(Event::Start(e)) => return Ok(e.name().as_ref() == b"feed"),
                Ok(Event::Eof) => return Err("Invalid feed".into()),
                _ => continue,
            }
        }
    }

    pub async fn new(url: &str) -> Result<Self, Box<dyn Error>> {
        let content = reqwest::get(url).await?.bytes().await?;
        let is_atom = Self::is_atom(content.as_ref())?;
        let articles = if is_atom {
            atom_syndication::Feed::read_from(content.as_ref())?
                .entries
                .into_iter()
                .map(Article::Atom)
                .collect()
        } else {
            Channel::read_from(content.as_ref())?
                .items
                .into_iter()
                .map(Article::Rss)
                .collect()
        };

        Ok(Self { articles })
    }
}

impl Article {
    pub fn title(&self) -> String {
        match self {
            Article::Atom(a) => a.title().value.clone(),
            Article::Rss(a) => a.title().unwrap_or_default().to_string(),
        }
    }

    pub fn url(&self) -> String {
        match self {
            Article::Atom(a) => a.links().first().map_or("".to_string(), |i| i.href.clone()),
            Article::Rss(a) => a.link().unwrap_or_default().to_string(),
        }
    }

    pub fn host(url: &str) -> String {
        Url::parse(url).map_or("".to_string(), |i| {
            i.host_str().unwrap_or_default().to_string()
        })
    }

    pub fn time(&self) -> DateTime<FixedOffset> {
        match self {
            Article::Atom(a) => a.published.unwrap_or_default(),
            Article::Rss(a) => a.pub_date.as_ref().map_or(DateTime::default(), |i| {
                DateTime::parse_from_rfc2822(i).unwrap_or_default()
            }),
        }
    }
}
