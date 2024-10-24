use hdf5_nuc_data_reader::{read_hdf5};

fn main() {
    // Define the filename
    const FILENAME: &str = "../Li6.h5";
    // Define the temperature and reaction keys you want to print
    let temperature_key = "294K";
    let reaction_key = "reaction_444";

    // Call the read_hdf5 function with the filename as an argument
    let (energy_map, reactions_map) = read_hdf5(FILENAME);

    // Print the key structure of the returned HashMaps
    println!("Energy map keys:");
    for key in energy_map.keys() {
        println!("  {}", key);
    }

    println!("Reactions map keys:");
    for (reaction, temp_map) in &reactions_map {
        println!("  Reaction: {}", reaction);
        for temp in temp_map.keys() {
            println!("    Temperature: {}", temp);
        }
    }

    // Print the energy values for the specified temperature
    println!(
        "Energy data for {}: {:?}", 
        temperature_key, 
        energy_map.get(temperature_key).unwrap()
    );

    // Print the reaction data for the specified temperature and reaction
    println!(
        "Reaction {} data for {}: {:?}", 
        reaction_key, 
        temperature_key, 
        reactions_map.get(reaction_key).unwrap().get(temperature_key).unwrap()
    );
}