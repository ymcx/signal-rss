use atom_syndication::Entry;
use chrono::{DateTime, FixedOffset};
use quick_xml::{Reader, events::Event};
use rss::{Channel, Item};
use std::error::Error;

pub struct Feed {
    articles: Vec<Article>,
}

pub enum Article {
    Atom(Entry),
    Rss(Item),
}

impl Feed {
    fn is_atom(content: &[u8]) -> Option<bool> {
        let content = std::str::from_utf8(content).unwrap_or_default();
        let mut reader = Reader::from_str(content);

        loop {
            match reader.read_event() {
                Ok(Event::Start(e)) => return Some(e.name().as_ref() == b"feed"),
                Ok(Event::Eof) => return None,
                _ => continue,
            }
        }
    }

    pub async fn new(url: &str) -> Result<Self, Box<dyn Error>> {
        let content = &reqwest::get(url).await?.bytes().await?[..];
        let atom = Self::is_atom(content);
        if atom.is_none() {
            println!("voivutt");
            return Err("".into());
        }
        let atom = atom.unwrap();
        println!("is atom {}", atom);

        let articles: Vec<Article> = if atom {
            atom_syndication::Feed::read_from(content)?
                .entries
                .iter()
                .map(|i| Article::Atom(i.clone()))
                .collect()
        } else {
            Channel::read_from(content)?
                .items
                .iter()
                .map(|i| Article::Rss(i.clone()))
                .collect()
        };
        Ok(Self { articles })
    }

    pub fn get(self) -> Vec<Article> {
        self.articles
    }
}

impl Article {
    pub fn title(&self) -> String {
        match self {
            Article::Atom(entry) => entry.title.value.clone(),
            Article::Rss(entry) => entry.title.clone().unwrap_or_default(),
        }
    }

    pub fn url(&self) -> String {
        match self {
            Article::Atom(entry) => entry.links.get(0).unwrap().href.clone(),
            Article::Rss(entry) => entry.link.clone().unwrap(),
        }
    }

    pub fn time(&self) -> DateTime<FixedOffset> {
        match self {
            Article::Atom(entry) => entry.published.unwrap_or_default(),
            Article::Rss(entry) => entry
                .pub_date
                .as_ref()
                .and_then(|d| DateTime::parse_from_rfc2822(d).ok())
                .unwrap(),
        }
    }
}
