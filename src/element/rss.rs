use crate::common::*;
// STD
use std::io::Read;
// External
use rss;

pub fn site(s: &str) -> Result<ElementsMin, CustomError> {
    // WARN: Could I avoid v?
    let mut v = Vec::new();
    match ureq::get(s).call() {
        Ok(s) => match s.into_reader().read_to_end(&mut v) {
            Err(_) => return Err(CustomError::Internet),
            Ok(_) => (),
        },
        Err(_) => return Err(CustomError::InvalidLink),
    };
    let r = rss::Channel::read_from(&*v).unwrap();
    // Collect return
    let mut ret = Vec::new();
    for i in r.items() {
        // Get datetime from rss channel
        let dt = match i.pub_date.as_ref() {
            Some(x) => x,
            None => return Err(CustomError::Datetime),
        };
        let date: DateTime<Local> = match DateTime::parse_from_rfc2822(&dt) {
            Err(_) => return Err(CustomError::Datetime),
            Ok(o) => o.with_timezone(&Local),
        };
        // WARN: This could be done without cloning
        let title = i.title.clone().unwrap_or("No title".to_string());
        let content = i.content.clone();
        let kind = ElementKind::RssItem(r.link.clone());
        ret.push(ElementMin {
            title,
            content,
            date,
            kind
        });
    }
    // Sort by date & return
    ret.sort();
    Ok(ret)
}
