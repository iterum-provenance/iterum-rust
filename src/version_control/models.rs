use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Dataset {
    pub commits: HashMap<String, Commit>,
    pub branches: HashMap<String, Branch>,
    pub version_tree: VersionTree,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ChangeType {
    Added,
    Removed,
    Updated,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Diff {
    pub added: Vec<String>,
    pub removed: Vec<String>,
    pub updated: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Deprecated {
    pub value: bool,
    pub reason: String,
}

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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Branch {
    pub hash: String,
    pub name: String,
    pub head: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VersionTreeNode {
    pub name: String,
    pub branch: String,
    pub children: Vec<String>,
    pub parent: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VersionTree {
    pub tree: HashMap<String, VersionTreeNode>,
    pub branches: HashMap<String, String>,
}
