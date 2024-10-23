use hdf5::File;
// use ndarray::ArrayBase;
use std::path::Path;

fn read_hdf5(filename: &str, temperature_key: &str, reaction_key:  &str,) -> (Vec<f64>, Vec<f64>) {
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

    
    let group_reactions_mt = group_reactions.group(&reaction_key).expect("Failed to open dataset");
    let group_294k = group_reactions_mt.group(temperature_key).expect("Failed to open group 294K");
    let dataset_xs = group_294k.dataset("xs").expect("Failed to open dataset xs");
    
    println!("{:?}", dataset_xs);

    let dataset_energy = group_energy.dataset(&temperature_key).expect("Failed to open dataset");
    
    let energy_data: Vec<f64> = dataset_energy.read_1d().expect("Failed to read dataset").to_vec();
    let mt_data: Vec<f64> = dataset_xs.read_1d().expect("Failed to read dataset").to_vec();

    (energy_data, mt_data)
}

fn main() {
    // Define the filename and dataset key
    const FILENAME: &str = "Li6.h5";
    const TEMPERATURE: &str = "294K";
    const REACTION_MT: &str = "reaction_002";

    // Call the read_hdf5 function with the filename and dataset key as arguments
    let (dataset_energy, data) = read_hdf5(FILENAME, TEMPERATURE, REACTION_MT);
    // read_hdf5(FILENAME, DATASET_KEY);

    // Print the energy values
    println!("Energy dataset: {:?}", dataset_energy);
    println!("Data: {:?}", data);
}
