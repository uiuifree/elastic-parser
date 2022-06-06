extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchResponse<T, V = Value> {
    #[serde(default)]
    pub _scroll_id: String,
    #[serde(default)]
    pub took: usize,
    pub hits: Hits<T>,
    #[serde(default)]
    pub aggregations: V,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HitsTotal {
    #[serde(default)]
    pub value: usize,
    #[serde(default)]
    pub relation: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Hits<T> {
    pub total: Option<HitsTotal>,
    pub max_score: Option<f32>,
    pub hits: Option<Vec<Hit<T>>>,
}

// impl<T> Hits<T> {
//     fn total(&self) -> HitsTotal {
//         return match self.total.clone() {
//             Some(v) => { v }
//             None => {
//                 HitsTotal {
//                     value: 0,
//                     relation: "".to_string(),
//                 }
//             }
//         };
//     }
//     fn max_score(&self) -> f32 {
//         self.max_score.unwrap_or(0.0)
//     }
// }

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Doc<T> {
    pub _index: String,
    pub _type: String,
    pub _id: String,
    // pub found:bool,
    // pub _version: Option<usize>,
    // pub _seq_no: Option<usize>,
    // pub _primary_term: Option<usize>,
    pub _source: Option<T>,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Shards{
pub total : Option<usize>,
pub successful : Option<usize>,
pub failed : Option<usize>,
}


use serde::de;
use serde_json::Value;

pub fn parse<'a, T>(s: &'a str) -> T
where
    T: de::Deserialize<'a>,
{
    serde_json::from_str(s).expect("")
}

impl<T> SearchResponse<T>
where
    T: std::clone::Clone,
{
    pub fn to_hit(&self) -> Vec<Hit<T>> {
        let hits = self.hits.clone();
        hits.hits.unwrap_or_default().to_vec()
    }
}
