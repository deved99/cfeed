// STD
use std::env::var;
// Local
pub use crate::element::{Element, ElementMin, ElementsMin, ElementKind, Elements};
pub use terminate::term;
// External
pub use chrono::{offset::Local, DateTime};
pub use serde_json as json;
pub use ureq;

// Error Handling
#[derive(Debug)]
pub enum CustomError {
    Internet,
    InvalidLink,
    InvalidName,
    Datetime,
    File(String)
}

pub fn get_config_path() -> String {
    match var("XFG_CONFIG_HOME") {
        Ok(s) => format!("{}/feeder/config.json", s),
        Err(_) => match var("HOME") {
            Ok(s) => format!("{}/.config/feeder/config.json", s),
            Err(_) => term!("$HOME not defined?")
        }
    }
}

pub fn format_long_lines(s: &str) -> String {
    let mut ret = String::new();
    for line in s.lines() {
        let mut c = 0;
        for w in line.split(" ") {
            ret += w;
            ret += " ";
            c += w.chars().count();
            if c > 40 {
                ret += "\n";
                c = 0
            }
        }
        if !line.is_empty() {
            ret = ret.trim().to_string();
        }
        if !ret.ends_with("\n") {
            ret += "\n";
        }
    }
    ret
}
