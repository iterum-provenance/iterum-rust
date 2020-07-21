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

#[derive(Serialize, Deserialize)]
pub enum StepStatus {
    Succeeded,
    Running,
    Failed,
}

#[derive(Serialize, Deserialize)]
pub struct PipelineExecution {
    pub pipeline_run: PipelineRun,
    pub status: HashMap<String, StepStatus>,
    pub results: Option<Vec<String>>,
}

impl PipelineRun {
    pub fn frag_name(&self) -> String {
        format!("{}-fragmenter", self.pipeline_run_hash)
    }
    pub fn comb_name(&self) -> String {
        format!("{}-combiner", self.pipeline_run_hash)
    }
    pub fn step_name(&self, step: &TransformationStep) -> String {
        format!("{}-{}", self.pipeline_run_hash, step.name.to_string())
    }

    pub fn create_output_channel_map(&self) -> HashMap<String, String> {
        let mut outputs = HashMap::new();
        for step in &self.steps {
            outputs.insert(step.output_channel.to_string(), self.step_name(step));
        }
        outputs.insert(self.fragmenter.output_channel.to_string(), self.frag_name());
        outputs
    }

    pub fn is_valid(&self) -> bool {
        // First map the inputs and output channels to each other
        let outputs = self.create_output_channel_map();

        // Determine whether there is a connection for all steps
        let mut first_node_upstream_map: HashMap<String, String> = HashMap::new();

        for step in &self.steps {
            match outputs.get(&step.input_channel) {
                Some(parent) => {
                    first_node_upstream_map.insert(self.step_name(step), parent.to_string());
                }
                None => {
                    return false;
                }
            };
        }
        match outputs.get(&self.combiner.input_channel) {
            Some(parent) => {
                first_node_upstream_map.insert(self.comb_name(), parent.to_string());
            }
            None => {
                return false;
            }
        };

        true
    }

    pub fn create_first_node_upstream_map(&self) -> HashMap<String, String> {
        // First map the inputs and output channels to each other
        let outputs = self.create_output_channel_map();

        // Determine whether there is a connection for all steps
        let mut first_node_upstream_map: HashMap<String, String> = HashMap::new();
        for step in &self.steps {
            let parent = outputs.get(&step.input_channel).unwrap();
            first_node_upstream_map.insert(self.step_name(step), parent.to_string());
        }
        let parent = outputs.get(&self.combiner.input_channel).unwrap();
        first_node_upstream_map.insert(self.comb_name(), parent.to_string());

        first_node_upstream_map
    }
}
