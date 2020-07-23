use serde::{Deserialize, Serialize};
use serde_json::value::Value;

fn empty_value() -> Value {
    Value::Null
}

/// Corresponds to the metadata in a fragment description (in the sidecars)
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Metadata {
    pub fragment_id: String,
    pub predecessors: Vec<String>,
    #[serde(default = "empty_value")]
    pub custom: Value,
}

/// Corresponds to the file description of the sidecars
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct FileDescription {
    pub name: String,
    pub path: String,
    pub bucket: String,
}

/// Corresponds to the fragment description of the sidecars
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct FragmentDescription {
    pub metadata: Metadata,
    pub files: Vec<FileDescription>,
}

/// Information which is saved to allow the reconstruction of lineage trees
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct FragmentLineage {
    pub transformation_step: String,
    pub description: FragmentDescription,
}
