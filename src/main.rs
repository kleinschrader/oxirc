extern crate yaml_rust;
use structopt::StructOpt;
use std::io::Read;

mod consts;
mod structs;

#[derive(StructOpt)]
struct Cli {
    /// Location where i can find the service files 'Default: /etc/oxirc/'
    #[structopt(short = "s", long = "svc-directory", default_value = "/etc/oxirc/")]
    svc_directory: String,
}

fn load_config_data(file_path: &str) -> Result<structs::Config, &'static str> {
    
    let mut config_file = match std::fs::File::open(file_path) {
        Ok(r) => r,
        Err(_) => {
            return Err("Error opening configfile")
        },
    };

    let mut config_contents = String::new();
    match config_file.read_to_string(&mut config_contents) {
        Ok(_) => (),
        Err(_) => {
            return Err("Error reading config contents")
        },
    }

    let config_data : structs::Config = match serde_yaml::from_str(&config_contents) {
        Ok(r) => r,
        Err(_) => {
            return Err("Error parsing Yaml")
        },
    };

    Ok(config_data)
}

fn load_units(unit_directory: &str) -> Vec<structs::UnitContainer> {
    for entry in std::fs::read_dir(unit_directory).unwrap() {
        let unit_entry = entry.unwrap();
        unit_entry
    }

    return Vec<structs::UnitContainer>();
}

#[tokio::main]
async fn main() -> std::io::Result<()> {

    let args = Cli::from_args();

    println!{"Working on {}", args.svc_directory};
    
    let config_file_path = format!("{}{}",args.svc_directory,consts::OXIRC_FILE);

    println!("Loading {}", config_file_path);
    
    let config_data = load_config_data(&config_file_path).expect("Error parsing yaml");


    loop
    {


        for stage in &config_data.boot_stages {
            println!("Working with stage: {}", stage);
        }


        std::thread::sleep(std::time::Duration::from_millis(500));
    }
}
       