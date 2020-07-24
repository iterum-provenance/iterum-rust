# Iterum Rust

This repository contains the code shared between the [daemon](https://github.com/iterum-provenance/daemon) and the [manager](https://github.com/iterum-provenance/manager). This library is split into three submodules: pipeline-, provenance- and version-control- related structures. For the version-control module it also includes the logic with regards to integrity checks for the dataset versions.

A general overview of the Iterum framework can be found [here](https://github.com/iterum-provenance/iterum).

# Tests

For the integrity checking of commits being added to a dataset, some tests have been written. These tests can be run by running the following command:
```
cargo test
```
This will show which tests have succeeded and which tests have failed.
