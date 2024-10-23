use hdf5::File;
// use ndarray::ArrayBase;
use std::path::Path;

fn read_hdf5(filename: &str, dataset_key: &str) -> Vec<i64> {
    // Open the HDF5 file
    let file = File::open(Path::new(filename)).expect("Failed to open HDF5 file");

    let group_names: Vec<String> = file.member_names().expect("Failed to get group names");
    
    for name in &group_names {
        println!("Group name: {}", name);  // this prints Li6 if the Li6.h5 file is loaded
    }
    // let names=
    // Navigate through groups and read the dataset
    let group_nuclide = file.group(&group_names[0]).expect("Failed to open group Li6");
    
    let nuclide_names = group_nuclide.member_names().expect("Failed to get group names");
    println!("{:?}", nuclide_names);
    
    let group_temperature = group_nuclide.group("kTs").expect("Failed to open group energy");
    let temperature_group_names: Vec<String> = group_temperature.member_names().expect("Failed to get group names");
    println!("{:?}", temperature_group_names);

    let group_energy = group_nuclide.group("energy").expect("Failed to open group energy");
    let group_reactions = group_nuclide.group("reactions").expect("Failed to open group energy");
    let group_reactions_names: Vec<String> = group_reactions.member_names().expect("Failed to get group names");
    println!("{:?}", group_reactions_names);

    let dataset = group_energy.dataset(&dataset_key).expect("Failed to open dataset");
    let data: Vec<i64> = dataset.read_1d().expect("Failed to read dataset").to_vec();

    data
}

fn main() {
    // Define the filename and dataset key
    const FILENAME: &str = "Li6.h5";
    const DATASET_KEY: &str = "294K";

    // Call the read_hdf5 function with the filename and dataset key as arguments
    let data = read_hdf5(FILENAME, DATASET_KEY);
    // read_hdf5(FILENAME, DATASET_KEY);

    // Print the energy values
    println!("{:?}", data);
}
