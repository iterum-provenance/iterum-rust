use crate::version_control::{
    Branch, Commit, Dataset, Deprecated, Diff, VersionTree, VersionTreeNode,
};
// use crate::pipeline::models::PipelineResult;
use crate::utils::create_random_hash;
use std::collections::HashMap;

impl Default for Dataset {
    fn default() -> Self {
        Self::new()
    }
}

impl Dataset {
    /// Creates a new dataset, and populates it with a root commit and master branch
    pub fn new() -> Dataset {
        let mut tree: HashMap<String, VersionTreeNode> = HashMap::new();
        let root_commit_hash = create_random_hash();
        let master_branch_hash = create_random_hash();
        let root_commit = Commit {
            hash: root_commit_hash.to_string(),
            parent: None,
            branch: master_branch_hash.to_string(),
            name: "root".to_owned(),
            description: "".to_owned(),
            files: vec![],
            diff: Diff {
                added: vec![],
                updated: vec![],
                removed: vec![],
            },
            deprecated: Deprecated {
                value: false,
                reason: "".to_owned(),
            },
        };
        let vtree_root_node = VersionTreeNode {
            name: "root".to_owned(),
            branch: master_branch_hash.to_string(),
            children: vec![],
            parent: None,
        };
        tree.insert(root_commit_hash.to_string(), vtree_root_node);
        let master_branch = Branch {
            hash: master_branch_hash.to_string(),
            name: "master".to_owned(),
            head: root_commit_hash.to_string(),
        };
        let mut branches = HashMap::new();
        branches.insert(master_branch.hash.clone(), "master".to_owned());
        let version_tree = VersionTree { tree, branches };

        let mut commit_map: HashMap<String, Commit> = HashMap::new();
        commit_map.insert(root_commit_hash, root_commit);

        let mut branch_map: HashMap<String, Branch> = HashMap::new();
        branch_map.insert(master_branch_hash, master_branch);

        Dataset {
            commits: commit_map,
            branches: branch_map,
            version_tree,
        }
    }
}
