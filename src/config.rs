// STD
use std::collections::HashMap;
use std::fs;
// Local
use crate::common::*;
use crate::element::{self, ElementSrc};
// External crates
use serde::{Deserialize, Serialize};
use serde_json as json;

type Sources = Vec<ElementSrc>;
#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    categories: HashMap<String, Vec<String>>,
    sources: HashMap<String, Sources>,
    opener: HashMap<String, String>
}
impl Config {
    pub fn load(&self) -> LoadedConfig {
        let res: Result<Vec<Elements>, CustomError> = self
            .sources
            .iter()
            .map(|(k, v)| v.iter().map(move |x| {
                let o = match x {
                    ElementSrc::Youtube(_) => &self.opener["youtube"],
                    ElementSrc::Twitter(_) => &self.opener["twitter"],
                    ElementSrc::RSS(_) => &self.opener["rss"]
                };
                x.load(k, o)
            }))
            .flatten()
            .collect();
        let elems = res.unwrap();
        LoadedConfig {
            categories: self.categories.clone(),
            elements: element::merge(elems),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoadedConfig {
    pub categories: HashMap<String, Vec<String>>,
    pub elements: Elements,
}
impl LoadedConfig {
    pub fn load() -> Result<LoadedConfig,CustomError> {
        let f = match fs::File::open( get_config_path() ) {
            Err(why) => return Err(CustomError::File(why.to_string())),
            Ok(s) => s
        };
        let c: Config = match json::from_reader(&f) {
            Err(why) => return Err(CustomError::File(why.to_string())),
            Ok(c) => c
        };
        Ok(c.load())
    }
}
