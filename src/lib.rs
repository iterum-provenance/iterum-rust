//! This crate consists of mostly structs which are shared between the daemon and the manager.
//! It is split up into three submodules: pipeline-, provenance- and version-control- related structures.
//! For the version-control module it also includes the logic with regards to integrity checks for the
//! dataset versions.

pub mod pipeline;
pub mod utils;
pub mod version_control;

// Shorthand, such that you don't have to type version_control all the time
pub use version_control as vc;
pub mod provenance;
