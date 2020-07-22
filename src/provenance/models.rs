use serde::{Deserialize, Serialize};
use serde_json::value::Value;

pub fn empty_value() -> Value {
    Value::Null
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Metadata {
    pub fragment_id: String,
    pub predecessors: Vec<String>,
    #[serde(default = "empty_value")]
    pub custom: Value,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct FileDescription {
    pub name: String,
    pub path: String,
    pub bucket: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct FragmentDescription {
    pub metadata: Metadata,
    pub files: Vec<FileDescription>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct FragmentLineage {
    pub transformation_step: String,
    pub description: FragmentDescription,
}
