//! This module consists of the structures regarding data versioning, as well as the logic for checking whether versions of data sets are valid.
//!
//! It contains structures which describes the metadata of a dataset: All the data related to the versioning, and how versions relate to each other,
//! except the data stored in the dataset themselves.
//!
//! The general idea is that each modification function is stateless: The functions to modify a dataset consume a dataset, and return a modified dataset.
//! This makes the checking for integrity easier, and tests can be written more easily.
//! Functions therefore have a signature similar to this:
//! fn(Dataset, modification) -> Result<Modified dataset, Error>
//!
pub mod dataset;
pub mod error;

pub mod branch;
pub mod commit;
pub mod models;
pub mod tests;

pub use models::{
    Branch, ChangeType, Commit, Dataset, Deprecated, Diff, VersionTree, VersionTreeNode,
};
