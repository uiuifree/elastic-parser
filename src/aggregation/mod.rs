use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct AggregationResponseParser {
    value: Value,
}

impl AggregationResponseParser {
    pub fn new(value: Value) -> AggregationResponseParser {
        AggregationResponseParser {
            value
        }
    }
    pub fn value(&self) -> Value {
        self.value.clone()
    }
    pub fn doc_count(&self) -> Option<u64> {
        parse_u32(self.value.get("doc_count"))
    }
    pub fn sum_other_doc_count(&self) -> Option<u64> {
        parse_u32(self.value.get("sum_other_doc_count"))
    }
    pub fn sub_aggregation<T: Into<String>>(&self, key: T) -> Option<AggregationResponseParser> {
        let agg = self.value.get(key.into())?.clone();
        Some(AggregationResponseParser::new(agg))
    }
    pub fn buckets(&self) -> Option<Vec<AggregationResponseBucket>> {
        let values = self.value.get("buckets")?.clone();
        let values = values.as_array()?.clone();
        let mut response = vec![];
        for value in values {
            response.push(AggregationResponseBucket::new(value))
        }
        Some(response)
    }
}


#[derive(Debug, Default, Clone)]
pub struct AggregationResponseBucket {
    value: Value,
}

impl AggregationResponseBucket {
    pub fn new(value: Value) -> AggregationResponseBucket {
        AggregationResponseBucket {
            value
        }
    }
    pub fn key(&self) -> Option<String> {
        let key_as_string = self.value.get("key_as_string");
        if key_as_string.is_some() {
            let key_as_string = key_as_string.unwrap();
            if key_as_string.is_string() {
                return Some(key_as_string.as_str().unwrap().to_string());
            }
        }
        let key = self.value.get("key");
        if key.is_some() {
            let key = key.unwrap();
            if key.is_string() {
                return Some(key.as_str().unwrap().to_string());
            }
            if  key.is_number(){
                return Some(key.as_str().unwrap().to_string());
            }
        }
        return None;
    }
    pub fn keys(&self) -> Option<Vec<String>> {
        let key = self.value.get("key");
        if !key.is_some() {
            return None;
        }
        let key = key.unwrap();
        if key.is_string() || key.is_number() {
            return Some(vec![key.to_string()]);
        }
        if key.is_array() {
            let keys = key.as_array().unwrap();
            let mut response = vec![];
            for key in keys {
                if key.is_string() || key.is_number() {
                    response.push(key.to_string());
                }
            }
            return Some(response);
        }
        return None;
    }
    pub fn doc_count(&self) -> Option<u64> {
        parse_u32(self.value.get("doc_count"))
    }
}


fn parse_u32(value: Option<&Value>) -> Option<u64> {
    if value.is_none() {
        return None;
    }
    let value = value.unwrap();
    if value.is_u64() {
        return Some(value.as_u64().unwrap());
    }
    return None;
}


// #[derive(Debug, Default, Clone, Deserialize, Serialize)]
// pub struct ResponseTermAggregation<T: Default = ResponseBucket> {
//     pub doc_count_error_upper_bound: u32,
//     pub sum_other_doc_count: u32,
//     pub buckets: Vec<T>,
// }
//
// #[derive(Debug, Default, Clone, Deserialize, Serialize)]
// pub struct ResponseBucket {
//     pub key: Option<String>,
//     pub doc_count: u32,
// }
//
// #[derive(Debug, Default, Clone, Deserialize, Serialize)]
// pub struct ResponseCustomBucket<T> {
//     pub key: String,
//     pub doc_count: i32,
//     pub value: T,
// }
//
// #[derive(Debug, Default, Clone, Deserialize, Serialize)]
// pub struct ResponseMultiTermAggregation<T: Default = ResponseMultiBucket> {
//     pub doc_count_error_upper_bound: u32,
//     pub sum_other_doc_count: u32,
//     pub buckets: Vec<T>,
// }
//
// #[derive(Debug, Default, Clone, Deserialize, Serialize)]
// pub struct ResponseMultiBucket {
//     pub key_as_string: String,
//     pub doc_count: u32,
// }
//
// #[derive(Debug, Default, Clone, Deserialize, Serialize)]
// pub struct ResponseCardinalityAggregation {
//     pub value: u32,
// }
//
// #[derive(Debug, Default, Clone, Deserialize, Serialize)]
// pub struct ResponseStatsAggregation {
//     pub count: i32,
//     pub min: f64,
//     pub max: f64,
//     pub avg: f64,
//     pub sum: f64,
// }
