extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;


#[derive(Debug, Serialize, Deserialize)]
pub struct SearchResponse<T, V = Vec<String>> {
    pub took: i8,
    pub hits: Hits<T>,
    #[serde(default)]
    pub aggregations: V,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HitsTotal {
    #[serde(default)]
    pub value: usize,
    #[serde(default)]
    pub relation: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Hits<T> {
    pub total: HitsTotal,
    #[serde(default)]
    pub max_score: f32,
    pub hits: Vec<Hit<T>>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Hit<T> {
    #[serde(default)]
    pub _index: String,
    #[serde(default)]
    pub _type: String,
    #[serde(default)]
    pub _id: String,
    #[serde(default)]
    pub _score: f32,
    pub _source: T,
}

use serde::de;

pub fn parse<'a, T>(s: &'a str) -> T
    where
        T: de::Deserialize<'a>,
{
    serde_json::from_str(s).expect("")
}
