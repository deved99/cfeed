// STD
use std::rc::Rc;
use std::cmp::Ordering;
use std::process::{Command, Stdio};
// Local crates
mod rss;
mod twitter;
mod youtube;
use crate::common::CustomError;
// External
use chrono::{offset::Local, DateTime};
use itertools::Itertools;
use serde::{Deserialize, Serialize};

pub type Elements = Vec<Rc<Element>>;
pub type ElementsMin = Vec<ElementMin>;

pub fn merge(v: Vec<Elements>) -> Elements {
    // Add mutability
    let mut v = v;
    let n = v.len();
    if n > 1 {
        // Divide
        let half = n / 2;
        let other = v.drain(half..).collect();
        let a = merge(v);
        let b = merge(other);
        // Conquer
        merge_helper(a, b)
    } else {
        // Base case
        match v.pop() {
            Some(x) => x,
            None => Elements::new(),
        }
    }
}
fn merge_helper(a: Elements, b: Elements) -> Elements {
    let (mut a, mut b) = (a, b);
    b.drain(..).merge(a.drain(..)).collect()
}

#[derive(Debug, PartialEq, Eq)]
pub struct ElementMin {
    title: String,
    content: Option<String>,
    date: DateTime<Local>,
    kind: ElementKind,
}
impl ElementMin {
    pub fn to(self, from: &str, opener: &str) -> Element {
        let from = from.to_string();
        let opener = opener.to_string();
        Element {
            title: self.title,
            content: self.content,
            date: self.date,
            kind: self.kind,
            from, opener
        }
    }
}
impl PartialOrd for ElementMin {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        // Newest first
        Some(other.date.cmp(&self.date))
    }
}
impl Ord for ElementMin {
    fn cmp(&self, other: &Self) -> Ordering {
        other.date.cmp(&self.date)
    }
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Element {
    title: String,
    content: Option<String>,
    date: DateTime<Local>,
    kind: ElementKind,
    from: String,
    opener: String
}
impl Element {
    pub fn pretty(&self) -> String {
        format!(
            "<b>[{}] {}</b>\n{}",
            self.date.format("%R of %x"),
            &self.from,
            &self.title
        )
    }
    pub fn from(&self) -> &String {
        &self.from
    }
    pub fn open(&self) -> String {
        use ElementKind::*;
        let c = match &self.kind {
            RssItem(s) => s.clone(),
            Tweet(s) => format!("twitter.com/anyone/status/{}", s),
            YoutubeVideo(s) => format!("https://www.youtube.com/watch?v={}", s),
        };
        cmd(&self.opener, &c);
        format!("{} {}", &self.opener, &c)

    }
}
impl PartialOrd for Element {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        // Newest first
        Some(other.date.cmp(&self.date))
    }
}
impl Ord for Element {
    fn cmp(&self, other: &Self) -> Ordering {
        other.date.cmp(&self.date)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ElementKind {
    RssItem(String),
    Tweet(String),
    YoutubeVideo(String),
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ElementSrc {
    RSS(String),
    Twitter(String),
    Youtube(String),
}
impl ElementSrc {
    pub fn load(&self, from: &str, opener: &str) -> Result<Elements, CustomError> {
        use ElementSrc::*;
        let r = match &self {
            RSS(s) => rss::site(s),
            Twitter(s) => twitter::profile(s),
            Youtube(s) => youtube::channel(s),
        };
        match r {
            Err(e) => Err(e),
            Ok(mut v) => {
                let mut r = Vec::new();
                while let Some(i) = v.pop() {
                    r.push(Rc::new(i.to(from, opener)))
                }
                r.reverse();
                Ok(r)
            }
        }
    }
}

// Helper
fn cmd(p: &str, a: &str) {
    match Command::new(p)
        .arg(a)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
    {
        Err(why) => println!("Failed running {} with {} as arg:\n{}", p, a, why),
        Ok(_) => (),
    }
}
