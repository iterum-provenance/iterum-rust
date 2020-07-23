use crate::utils::create_random_hash;
use crate::vc::{Commit, Dataset, Deprecated, Diff};

pub fn create_new_dataset() -> Dataset {
    Dataset::new()
}

pub fn create_dummy_commit(dataset: &Dataset) -> Commit {
    let branch = dataset
        .branches
        .iter()
        .find(|(_, branch)| branch.name == "master")
        .unwrap()
        .1;
    let mut commit = Commit {
        hash: "".to_owned(),
        name: "".to_owned(),
        parent: Some("".to_owned()),
        branch: "".to_owned(),
        description: "".to_owned(),
        deprecated: Deprecated {
            value: false,
            reason: "".to_owned(),
        },
        diff: Diff {
            added: vec![],
            updated: vec![],
            removed: vec![],
        },
        files: vec![],
    };

    commit.branch = branch.hash.to_string();
    commit.hash = create_random_hash();
    commit.parent = Some(branch.head.to_string());

    commit
}
