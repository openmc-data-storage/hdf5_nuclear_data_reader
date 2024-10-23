use hdf5::File;
use std::collections::HashMap;
use std::path::Path;

fn read_hdf5(filename: &str) -> HashMap<String, HashMap<String, HashMap<String, Vec<f64>>>> {
    // Open the HDF5 file
    let file = File::open(Path::new(filename)).expect("Failed to open HDF5 file");

    let group_names: Vec<String> = file.member_names().expect("Failed to get group names");
    
    for name in &group_names {
        println!("Group name: {}", name);  // this prints Li6 if the Li6.h5 file is loaded
    }

    // Navigate through groups and read the dataset
    let group_nuclide = file.group(&group_names[0]).expect("Failed to open group Li6");
    
    let nuclide_names = group_nuclide.member_names().expect("Failed to get group names");
    println!("{:?}", nuclide_names);
    
    let group_energy = group_nuclide.group("energy").expect("Failed to open group energy");
    let group_reactions = group_nuclide.group("reactions").expect("Failed to open group reactions");
    let group_reactions_names: Vec<String> = group_reactions.member_names().expect("Failed to get group names");
    println!("{:?}", group_reactions_names);

    let temperature_group_names: Vec<String> = group_energy.member_names().expect("Failed to get temperature group names");
    println!("{:?}", temperature_group_names);

    let mut result = HashMap::new();
    let mut energy_map = HashMap::new();
    let mut reactions_map = HashMap::new();

    for temperature_key in temperature_group_names {
        let dataset_energy = group_energy.dataset(&temperature_key).expect("Failed to open dataset");
        let energy_data: Vec<f64> = dataset_energy.read_1d().expect("Failed to read dataset").to_vec();
        let mut temp_energy_map = HashMap::new();
        temp_energy_map.insert(temperature_key.clone(), energy_data);
        energy_map.insert(temperature_key.clone(), temp_energy_map);

        for reaction_name in &group_reactions_names {
            let group_reactions_mt = group_reactions.group(reaction_name).expect("Failed to open dataset");
            let group_temp = group_reactions_mt.group(&temperature_key).expect("Failed to open temperature group");
            let dataset_xs = group_temp.dataset("xs").expect("Failed to open dataset xs");
            let data: Vec<f64> = dataset_xs.read_1d().expect("Failed to read dataset").to_vec();
            reactions_map.entry(reaction_name.clone()).or_insert_with(HashMap::new).insert(temperature_key.clone(), data);
        }
    }

    result.insert("energy".to_string(), energy_map);
    result.insert("reactions".to_string(), reactions_map);

    result
}



fn main() {
    // Define the filename
    const FILENAME: &str = "Li6.h5";
    // Define the temperature and reaction keys you want to print
    let temperature_key = "294K";
    let reaction_key = "reaction_444";

    // Call the read_hdf5 function with the filename as an argument
    let result = read_hdf5(FILENAME);

    // Print the key structure of the returned HashMap
    for (key, value) in &result {
        println!("Top-level key: {}", key);
        for (sub_key, sub_value) in value {
            println!("  Second-level key: {}", sub_key);
            for (inner_key, _) in sub_value {
                println!("    Third-level key: {}", inner_key);
            }
        }
    }

    // Print the energy values for the specified temperature
    println!(
        "Energy data for {}: {:?}", 
        temperature_key, 
        result.get("energy").unwrap().get(temperature_key).unwrap().get(temperature_key).unwrap()
    );

    // Print the reaction data for the specified temperature and reaction
    println!(
        "Reaction {} data for {}: {:?}", 
        reaction_key, 
        temperature_key, 
        result.get("reactions").unwrap().get(reaction_key).unwrap().get(temperature_key).unwrap()
    );
}