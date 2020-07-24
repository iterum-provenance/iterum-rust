use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Structure which contains all the necessary information for versioned data sets
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Dataset {
    pub commits: HashMap<String, Commit>,
    pub branches: HashMap<String, Branch>,
    pub version_tree: VersionTree,
}

/// Possible ways in which a file can be changed
#[derive(Serialize, Deserialize, Debug)]
pub enum ChangeType {
    Added,
    Removed,
    Updated,
}

/// Actual changes in dataset for a commit
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Diff {
    pub added: Vec<String>,
    pub removed: Vec<String>,
    pub updated: Vec<String>,
}

/// Whether a commit should be used for pipeline runs or not
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Deprecated {
    pub value: bool,
    pub reason: String,
}

/// Version of a dataset. Only describes the version information, but the actual data from the dataset is stored somewhere else, dependent on the storage backend used.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Commit {
    pub hash: String,
    pub parent: Option<String>,
    pub branch: String,
    pub name: String,
    pub description: String,
    pub files: Vec<String>,
    pub diff: Diff,
    pub deprecated: Deprecated,
}

/// Description of a branch for the dataset
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Branch {
    pub hash: String,
    pub name: String,
    pub head: String,
}

/// A node for a version tree
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VersionTreeNode {
    pub name: String,
    pub branch: String,
    pub children: Vec<String>,
    pub parent: Option<String>,
}

/// Structure of how different versions in a dataset relate to each other
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VersionTree {
    pub tree: HashMap<String, VersionTreeNode>,
    pub branches: HashMap<String, String>,
}
