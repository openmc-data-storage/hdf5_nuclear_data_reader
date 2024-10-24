[![CI Rust testing](https://github.com/openmc-data-storage/hdf5_nuclear_data_reader/actions/workflows/ci.yml/badge.svg)](https://github.com/openmc-data-storage/hdf5_nuclear_data_reader/actions/workflows/ci.yml)

A portable hdf5 file reader that reads OpenMC cross section files.

This package incorporates the parts of hdf5 library needed for h5 file reading and therefore it is not necessary to install hdf5 on the computer before hand. See the Dockerfile for a demonstration of this

Ideally there will be several deployment options

- Cargo crate (in progress)
- WASM executable

For now the repo is no where near finished and very much experimental

First install Rust and Cargo. Instructions [here](https://doc.rust-lang.org/cargo/getting-started/installation.html)

The clone the repo

```
git clone https://github.com/shimwell/openmc_rust_hdf5_reader.git
```

Run the example using cargo
```
cd openmc_rust_hdf5_reader/examples_use
cargo run
```
