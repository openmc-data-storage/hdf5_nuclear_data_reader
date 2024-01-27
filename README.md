build binary with
```
cargo build
```

run with cargo using
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
