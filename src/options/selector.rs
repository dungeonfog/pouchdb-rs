use serde::Serialize;
use serde_json::{Map, Value};

#[derive(Default, Serialize, Debug)]
pub struct Selector(Map<String, Value>);
