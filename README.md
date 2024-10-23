# Work in progress - not ready for use

I plan to make a portable hdf5 file reader that reads OpenMC cross section files.

Ideally there will be several deployment options

- Cargo crate
- PyPi package
- WASM executable

For now the repo is no where near finished and very much experimental

First install Rust and Cargo. Instructions [here](https://doc.rust-lang.org/cargo/getting-started/installation.html)

The clone the repo

```
git clone https://github.com/shimwell/openmc_rust_hdf5_reader.git
```

build the Rust binary
```
cargo build
```

Run the binary
```
./target/debug/website_example
```

Run the binary using cargo (does not need building first)
```
cargo run
```


Python h5py can also access the cross section format and I've been using this to compare outputs

```python
import h5py

h5file = h5py.File('Li6.h5', 'r')

print(list(h5file['Li6']['energy']['294K']))
print(list(h5file['Li6']['reactions']))
print(list(h5file['Li6']['reactions']['reaction_444']['294K']['xs']))
print(list(h5file['Li6'].keys()))
```
