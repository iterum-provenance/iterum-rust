use std::error::Error;
use std::fmt;

#[derive(Debug, PartialEq)]
pub struct VCErrorMessage {
    message: String,
}

impl VCErrorMessage {
    pub fn new(message: String) -> VCErrorMessage {
        VCErrorMessage { message }
    }
}

#[derive(Debug, PartialEq)]
pub enum VersionControlError {
    CommitIncomplete(VCErrorMessage),
    ParentCommitNotFound,
    BranchNotFound,
    CommitNotFound,
    CommitHashAlreadyExists,
    BranchHashAlreadyExists,
    BranchHeadDoesNotExist,
    ParentCommitIsNotBranchHead,
    PipelineHashAlreadyExists,
}

impl Error for VersionControlError {}

impl fmt::Display for VersionControlError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            VersionControlError::CommitIncomplete(message) => {
                write!(f, "Commit is incomplete: {:?}", message)
            }
            VersionControlError::ParentCommitNotFound => {
                write!(f, "Parent of commit is not present in the version tree.")
            }
            VersionControlError::BranchNotFound => write!(f, "Branch not present in version tree."),
            VersionControlError::CommitNotFound => write!(f, "Commit not present in version tree."),
            VersionControlError::CommitHashAlreadyExists => {
                write!(f, "Commit hash already exists in the version tree.")
            }
            VersionControlError::BranchHashAlreadyExists => {
                write!(f, "Branch hash already exists in the version tree.")
            }
            VersionControlError::BranchHeadDoesNotExist => {
                write!(f, "Branch head does not exist in the version tree.")
            }
            VersionControlError::ParentCommitIsNotBranchHead => {
                write!(f, "The parent commit hash is not the head of the branch.")
            }
            VersionControlError::PipelineHashAlreadyExists => {
                write!(f, "The pipeline hash already exists.")
            }
        }
    }
}
