use std::io::Read;

#[path = "../structs.rs"]
mod structs;

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
    };

    Ok(unit_container)
}

pub fn load_units(unit_directory: &str) -> Vec<structs::UnitContainer> {
    
    let mut output_buffer: Vec<structs::UnitContainer> = std::vec::Vec::new();
    
    for entry in std::fs::read_dir(unit_directory).unwrap() {
        let unit_entry = entry.unwrap();
        let unit_entry_path: String = unit_entry.path().to_str().unwrap().to_string();

        if unit_entry.file_type().unwrap().is_dir() {
            let mut sub_result = load_units(&unit_entry_path);
            
            output_buffer.append(&mut sub_result)
        }
        else {
            let unit = load_unit_data(&unit_entry_path);
            output_buffer.extend(unit);
        }

    }

    return output_buffer;
}
