#[cfg(feature = "blosc")]
use hdf5::filters::blosc_set_nthreads;
use hdf5::{File, H5Type, Result};
use ndarray::{arr2, s};

#[derive(H5Type, Clone, PartialEq, Debug)] // register with HDF5
#[repr(u8)]
pub enum Color {
    R = 1,
    G = 2,
    B = 3,
}

#[derive(H5Type, Clone, PartialEq, Debug)] // register with HDF5
#[repr(C)]


fn read_hdf5() -> Result<()> {
    let file = File::open("Be9.h5")?; // open for reading
    let ds = file.dataset("Be9/energy/294K")
    // let ds2 = ds.dataset("energy")
    // .dataset("energy").dataset("294K")?; // open the dataset
    // ds.
    let ds2 = ds.read_slice(s![1.., ..]),
    println!("{ds}");
    
    Ok(())
}

fn main() -> Result<()> {
    read_hdf5()?;
    println!("finished");
    Ok(())
}
// f['Be9']['energy']['294K']
