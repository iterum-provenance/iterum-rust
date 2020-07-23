use super::error::VersionControlError;
use super::Branch;
use super::Dataset;

impl Dataset {
    pub fn add_branch(mut self, branch: &Branch) -> Result<Dataset, VersionControlError> {
        // Check whether the commit does not already exist:
        if self.branches.contains_key(&branch.hash) {
            return Err(VersionControlError::BranchHashAlreadyExists);
        }
        // Check whether the head of the branch exists:
        if !self.version_tree.tree.contains_key(&branch.head) {
            return Err(VersionControlError::BranchHeadDoesNotExist);
        }

        self.branches
            .insert(branch.hash.to_string(), branch.clone());

        self.version_tree
            .branches
            .insert(branch.hash.to_string(), branch.name.to_string());

        Ok(self)
    }
}

#[cfg(test)]
mod tests {
    use super::{Branch, Dataset};
    use crate::utils::create_random_hash;
    use crate::vc::tests::{create_dummy_commit, create_new_dataset};

    fn create_dummy_branch(dataset: &Dataset) -> Branch {
        let branch_hash = dataset.branches.iter().next().unwrap().0;
        let trunk = dataset.branches.get(branch_hash).unwrap();

        Branch {
            hash: create_random_hash(),
            name: "dummy".to_owned(),
            head: trunk.head.to_string(),
        }
    }

    #[test]
    fn add_branch() {
        let mut dataset = create_new_dataset();

        let commit1 = create_dummy_commit(&dataset);
        dataset = dataset.add_commit(&commit1).unwrap();
        let commit2 = create_dummy_commit(&dataset);
        dataset = dataset.add_commit(&commit2).unwrap();

        // Check if head of master is now commit2:
        let branch = dataset
            .branches
            .iter()
            .find(|(_, branch)| branch.name == "master")
            .unwrap()
            .1;
        assert!(
            branch.head == commit2.hash,
            "The head was not set correctly"
        );

        // Now add a branch from commit 1
        let mut branch = create_dummy_branch(&dataset);
        branch.head = commit1.hash;
        dataset = dataset.add_branch(&branch).unwrap();
        todo!("Fix this test");
    }
}
