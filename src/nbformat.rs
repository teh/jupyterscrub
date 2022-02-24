/// nbformat deserialiser. See
/// https://github.com/jupyter/nbformat/blob/master/nbformat/v4/nbformat.v4.schema.json
///
/// (de)serialising just the minimum necessary to scrub outputs.
use std::collections::BTreeMap;

use serde_json::Value;

type Any = BTreeMap<String, Value>;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Cell {
    // decoding cell_type as string because there can be arbitrary unreconised types
    pub cell_type: String,
    pub outputs: Option<Vec<Any>>,
    // pub execution_count: Option<i64>,
    #[serde(flatten)]
    pub additional_properties: Any,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Document {
    pub cells: Vec<Cell>,
    #[serde(flatten)]
    pub additional_properties: Any,
}
