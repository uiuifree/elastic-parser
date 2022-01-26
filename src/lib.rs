extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use std::io::Error;
use serde_json::Value;

#[derive(Debug, Serialize,Deserialize)]
pub struct SearchResponse<T, V = Vec<String>> {
    took: i8,
    hits: Hits<T>,
    #[serde(default)]
    aggregations: V,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HitsTotal {
    value: usize,
    relation: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Hits<T> {
    total: HitsTotal,
    max_score: f32,
    hits: Vec<Hit<T>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Hit<T> {
    _index: String,
    _type: String,
    _id: String,
    _score: f32,
    _source: T,
}

use serde::de;

pub fn parse<'a, T>(s: &'a str) -> T
    where
        T: de::Deserialize<'a>,
{
     serde_json::from_str(s).expect("")
}
