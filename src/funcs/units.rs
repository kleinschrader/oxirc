use std::io::Read;

#[path = "../structs.rs"]
pub mod structs;

pub use structs::{UnitContainer};

pub fn load_unit_data(unit_file_path: &str) -> Result<structs::UnitContainer, String> {
    let mut unit_file = match std::fs::File::open(unit_file_path) {
        Ok(r) => r,
        Err(e) => {
            return Err(format!("Error opening {}: {}", unit_file_path,e))
        },
    };

    let mut unit_data = String::new();
    match unit_file.read_to_string(&mut unit_data) {
        Ok(_) => (),
        Err(e) => {
            return Err(format!("Error reading {}: {}", unit_file_path,e))
        },
    };

    let unit_metadata = match std::fs::metadata(&unit_file_path) {
        Ok(r) => r,
        Err(e) => {
            return Err(format!("Error loading metadata of {}: {}", unit_file_path,e))
        },
    };

    let unit : structs::Unit = match serde_yaml::from_str(&unit_data) {
        Ok(r) => r,
        Err(e) => {
            return Err(format!("Error parsing {}: {}", unit_file_path,e))
        },
    };

    let unit_container : structs::UnitContainer = structs::UnitContainer {
        last_modifed: unit_metadata.modified().unwrap(),
        unit: unit,
        status: structs::UnitStatuses::Unknown,
    };

    Ok(unit_container)
}

pub fn load_units(unit_directory: &str) -> Vec<Box<structs::UnitContainer>> {
    
    let mut output_buffer: Vec<Box<structs::UnitContainer>> = std::vec::Vec::new();
    
    for entry in std::fs::read_dir(unit_directory).unwrap() {
        let unit_entry = entry.unwrap();
        let unit_entry_path: String = unit_entry.path().to_str().unwrap().to_string();

        if unit_entry.file_type().unwrap().is_dir() {
            let mut sub_result = load_units(&unit_entry_path);
            
            for x in sub_result {
                output_buffer.push(x);
            }
        }
        else {
            let unit = match load_unit_data(&unit_entry_path) {
                Ok(r) => output_buffer.push(Box::new(r)),
                Err(e) => {
                    println!("Error loading Unit {}: {}", unit_entry_path, e);
                }
            };

            
        }

    }

    return output_buffer;
}
