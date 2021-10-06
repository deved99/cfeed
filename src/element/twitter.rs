use std::env;
// Local
use crate::common::*;

const API: &str = "https://api.twitter.com/2";
const API_KEY: &str = env!("TWITTER_API","$TWITTER_API not defined");

pub fn profile(s: &str) -> Result<ElementsMin, CustomError> {
    let id = match user_id(s) {
        Ok(u) => u,
        Err(x) => return Err(x),
    };
    let u = format!("{}/users/{}/tweets?tweet.fields=created_at", API, id);
    let resp = match get(&u) {
        Ok(x) => x,
        Err(why) => return Err(why),
    };
    resp["data"]
        .as_array()
        .unwrap()
        .iter()
        .map(|x| {
            // Datetime handling
            let date = match DateTime::parse_from_rfc3339(x["created_at"].as_str().unwrap()) {
                Err(_) => return Err(CustomError::Datetime),
                Ok(d) => d.with_timezone(&Local),
            };
            // Everything else
            let title = x["text"].as_str().unwrap().to_string();
            let kind = ElementKind::Tweet(x["id"].as_str().unwrap().to_string());
            Ok(ElementMin {
                title,
                content: None,
                date,
                kind,
            })
        })
        .collect()
}

fn user_id(s: &str) -> Result<String, CustomError> {
    let u = format!("{}/users/by?usernames={}", API, s);
    match get(&u) {
        Err(why) => Err(why),
        Ok(o) => Ok(o["data"][0]["id"].as_str().unwrap().to_string()),
    }
}

fn get(u: &str) -> Result<json::Value, CustomError> {
    match ureq::get(&u).set("Authorization", API_KEY).call() {
        Err(_) => Err(CustomError::Internet),
        Ok(i) => Ok(i.into_json().unwrap()),
    }
}
