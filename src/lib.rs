extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchResponse<T, V = Value> {
    #[serde(default)]
    pub _scroll_id: Option<String>,
    #[serde(default)]
    pub took: Option<usize>,
    pub hits: Option<Hits<T>>,
    #[serde(default)]
    pub aggregations: Option<V>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HitsTotal {
    #[serde(default)]
    pub value: Option<usize>,
    #[serde(default)]
    pub relation: Option<String>,
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
    pub _index: Option<String>,
    #[serde(default)]
    pub _type: Option<String>,
    #[serde(default)]
    pub _id: Option<String>,
    #[serde(default)]
    pub _score: Option<f32>,
    pub _source: Option<T>,
}

impl<T: std::default::Default> Hit<T> {
    pub fn index(&self) -> String {
        if self._index.is_none(){
            return "".to_string();
        }
        self._index.as_ref().unwrap().to_string()
    }
    // pub fn get_type(&self) -> String {
    //     self.clone()._type.unwrap_or_default().to_string()
    // }
    // pub fn id(&self) -> String {
    //     self.clone()._id.unwrap_or_default().to_string()
    // }
    // pub fn score(&self) -> f32 {
    //     self.clone()._score.unwrap_or_default().clone()
    // }

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
pub struct Shards {
    pub total: Option<usize>,
    pub successful: Option<usize>,
    pub failed: Option<usize>,
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
        if hits.is_none() {
            return vec![];
        }
        hits.unwrap().hits.unwrap_or_default().to_vec()
    }
}
