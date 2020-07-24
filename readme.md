# Iterum Rust

This repository contains the code shared between the daemon and the manager. This library is split into three submodules: pipeline-, provenance- and version-control- related structures. For the version-control module it also includes the logic with regards to integrity checks for the dataset versions.

# Tests

For the integrity checking of commits being added to a dataset, some tests have been written. These tests can be run by running the following command:
```
cargo test
```
This will show which tests have succeeded and which tests have failed.