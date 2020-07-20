use crate::version_control::{
    Branch, Commit, Dataset, Deprecated, Diff, VersionTree, VersionTreeNode,
};
// use crate::pipeline::models::PipelineResult;
use crate::utils::create_random_hash;
use std::collections::HashMap;

// Dataset struct waar alle json files in zitten wat je ook op de disc opslaat.
// Dan kan je makkelijker integrity checks doen.
// Functies die een dataset krijgen + een aanpassing, en dan kan je zeggen of het mag of niet.
// Je kan dan een specifieke error terugsturen waarom iets niet mag.
// Verder kan je makkelijk unit tests schrijven voor de verschillende integrity checks.

// Je houd dan die dataset struct in de lokale cache. Je kan dan ook een RwLock daar opslaan.
// De data zou dan wel tot elke worker beschikbaar moeten zijn.

// Functies hebben dan de signature: Dataset + aanpassing -> Result<Aangepaste Dataset, Error>
// Vervolgens kan de caller de nieuwe dataset opslaan.

impl Default for Dataset {
    fn default() -> Self {
        Self::new()
    }
}

impl Dataset {
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
