use super::defaults::{empty_config, empty_hash, none_config_files_all, none_usize, one_instance};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TransformationStep {
    pub name: String,
    #[serde(default = "one_instance")]
    pub instance_count: usize,
    #[serde(default = "none_usize")]
    pub prefetch_count: Option<usize>,
    pub image: String,
    pub input_channel: String,
    pub output_channel: String,
    #[serde(default = "empty_config")]
    pub config: Config,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Fragmenter {
    pub image: String,
    pub output_channel: String,
    #[serde(default = "empty_config")]
    pub config: Config,
    #[serde(
        default = "none_config_files_all",
        skip_serializing_if = "Option::is_none"
    )]
    pub config_files_all: Option<HashMap<String, Vec<String>>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Combiner {
    pub input_channel: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    #[serde(default = "HashMap::new", skip_serializing_if = "HashMap::is_empty")]
    pub config_files: HashMap<String, String>,
    #[serde(default = "HashMap::new", flatten)]
    pub config: HashMap<String, Value>,
}

impl Config {
    pub fn is_empty(&self) -> bool {
        self.config.is_empty() && self.config_files.is_empty()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PipelineRun {
    #[serde(default = "empty_hash")]
    pub pipeline_run_hash: String,
    pub name: String,
    pub input_dataset: String,
    pub input_dataset_commit_hash: String,
    pub fragmenter: Fragmenter,
    pub steps: Vec<TransformationStep>,
    pub combiner: Combiner,
    #[serde(default = "empty_config", skip_serializing_if = "Config::is_empty")]
    pub config: Config,
}
