//! This module consists of the structures regarding data versioning, as well as the logic for checking whether versions of data sets are valid.
pub mod dataset;
pub mod error;

pub mod branch;
pub mod commit;
pub mod models;
pub mod tests;

pub use models::{
    Branch, ChangeType, Commit, Dataset, Deprecated, Diff, VersionTree, VersionTreeNode,
};
