// Local
use crate::common::*;

// Constants
const YT_API: &str = "https://www.googleapis.com/youtube/v3";
const API_KEY: &str = env!("GOOGLE_API","$GOOGLE_API not defined");

pub fn channel(s: &str) -> Result<ElementsMin, CustomError> {
    let v = vec![("part", "snippet"), ("playlistId", s)];
    let o = match get("playlistItems", &v) {
        Err(why) => return Err(why),
        Ok(x) => x,
    };
    let ret = match o["items"].as_array() {
        None => return Err(CustomError::InvalidName),
        Some(x) => x,
    };
    ret.iter()
        .map(|i| {
            let s = &i["snippet"];
            // Datetime
            let date = match DateTime::parse_from_rfc3339(s["publishedAt"].as_str().unwrap()) {
                Err(_) => return Err(CustomError::Datetime),
                Ok(d) => d.with_timezone(&Local),
            };
            // Everything else
            let title = s["title"].as_str().unwrap().to_string();
            let content = Some(s["description"].as_str().unwrap().to_string());
            let kind =
                ElementKind::YoutubeVideo(s["resourceId"]["videoId"].as_str().unwrap().to_string());
            Ok(ElementMin {
                title,
                content,
                date,
                kind,
            })
        })
        .collect()
}

fn get(s: &str, args: &Vec<(&str, &str)>) -> Result<json::Value, CustomError> {
    let arg = args
        .iter()
        .map(|x| format!("&{}={}", x.0, x.1))
        .collect::<Vec<String>>()
        .join("");
    let s = format!("{}/{}?key={}{}", YT_API, s, API_KEY, arg);
    match ureq::get(&s).call() {
        Err(_) => Err(CustomError::Internet),
        Ok(o) => Ok(o.into_json().unwrap()),
    }
}
