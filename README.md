First install Rust and Cargo. Instructions [here]([url](https://doc.rust-lang.org/cargo/getting-started/installation.html))

I plan to make a Carog crete but till then clone this repo

```
git clone https://github.com/shimwell/openmc_rust_hdf5_reader.git
```

build the Rust binary
```
cargo build
```

Run the binary using cargo
```
cargo run
```

execute with
```
./target/debug/website_example
```

Python h5py can also access the cross section format
```python
import h5py

h5file = h5py.File('Li6.h5', 'r')

print(list(h5file['Li6']['energy']['294K']))
print(list(h5file['Li6']['reactions']))
print(list(h5file['Li6']['reactions']['reaction_444']['294K']['xs']))
print(list(h5file['Li6'].keys()))
```
