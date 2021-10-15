extern crate yaml_rust;
use structopt::StructOpt;
use std::io::Read;

mod consts;
mod structs;

mod funcs;

use funcs::units::load_units;
use funcs::units::structs::UnitStatuses;


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

fn unit_runner(mut unit: Box<funcs::units::UnitContainer>) -> () {

    let username: &str = &unit.as_ref().unit.runas;
    
    let user = nix::unistd::User::from_name(username).unwrap().unwrap();

    loop {

        match nix::unistd::seteuid(user.uid) {
            Ok(_) => (),
            Err(_) => {
                unit.as_mut().status = UnitStatuses::ConfigError;
                return ()
            },
        };

        std::process::Command::new(&unit.as_ref().unit.command)
            .spawn().unwrap();

        std::thread::sleep(std::time::Duration::from_millis(10000));
    }
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
        match unsafe{ nix::unistd::fork() } {
            Ok(nix::unistd::ForkResult::Child) => {
                unit_runner(unit);
            }
            Ok(_) => {

            }
            Err(_)  => {

            }
        }
    }

    loop
    {



        std::thread::sleep(std::time::Duration::from_millis(500));
    }
}
       