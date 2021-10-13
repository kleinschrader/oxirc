extern crate yaml_rust;
use structopt::StructOpt;
use std::io::Read;

mod consts;
mod structs;

mod funcs;

use funcs::units::load_units;


#[derive(StructOpt)]
struct Cli {
    /// Location where i can find the service files 'Default: /etc/oxirc/'
    #[structopt(short = "s", long = "svc-directory", default_value = "/etc/oxirc/")]
    svc_directory: String,
}

fn load_config_data(file_path: &str) -> Result<structs::Config, String> {
    
    let mut config_file = match std::fs::File::open(file_path) {
        Ok(r) => r,
        Err(e) => {
            return Err(format!("Error opening configfile: {}", e))
        },
    };

    let mut config_contents = String::new();
    match config_file.read_to_string(&mut config_contents) {
        Ok(_) => (),
        Err(e) => {
            return Err(format!("Error reading config contents: {}",e))
        },
    }

    let config_data : structs::Config = match serde_yaml::from_str(&config_contents) {
        Ok(r) => r,
        Err(e) => {
            return Err(format!("Error parsing Yaml: {}",e))
        },
    };

    Ok(config_data)
}


#[tokio::main]
async fn main() -> std::io::Result<()> {

    let args = Cli::from_args();

    println!{"Working on {}", args.svc_directory};
    
    let config_file_path = format!("{}{}",args.svc_directory,consts::OXIRC_FILE);
    let unit_file_path = format!("{}{}",args.svc_directory,"units/");

    println!("Loading {}", config_file_path);
    
    let config_data = load_config_data(&config_file_path).expect("Error parsing yaml");

    let units = load_units(&unit_file_path);

    for unit in units {
        println!("{}",unit.unit.name);
    }

    loop
    {


        for stage in &config_data.boot_stages {
            println!("Working with stage: {}", stage);
        }


        std::thread::sleep(std::time::Duration::from_millis(500));
    }
}
       