use hdf5::File;
use ndarray::ArrayBase;
use std::path::Path;

fn read_hdf5(filename: &str, dataset_key: &str) -> Vec<i64> {
    // Open the HDF5 file
    let file = File::open(Path::new(filename)).expect("Failed to open HDF5 file");

    let group_names: Vec<String> = file.member_names().expect("Failed to get group names");
    
    for name in &group_names {
        println!("Group name: {}", name);
    }
    // let names=
    // Navigate through groups and read the dataset
    let group_nuclide = file.group(&group_names[0]).expect("Failed to open group Li6");
    let group_energy = group_nuclide.group("energy").expect("Failed to open group energy");
    let group_temperature = group_nuclide.group("kTs").expect("Failed to open group energy");

    let temperature_group_names: Vec<String> = group_temperature.member_names().expect("Failed to get group names");
    
    println!("{:?}", temperature_group_names[0]);

    let dataset = group_energy.dataset(&temperature_group_names[0]).expect("Failed to open dataset");
    let data: Vec<i64> = dataset.read_1d().expect("Failed to read dataset").to_vec();

    data
}

fn main() {
    // Define the filename and dataset key
    const FILENAME: &str = "Li6.h5";
    const DATASET_KEY: &str = "294K";

    // Call the read_hdf5 function with the filename and dataset key as arguments
    let data = read_hdf5(FILENAME, DATASET_KEY);

    // Print the result
    println!("{:?}", data);
}
