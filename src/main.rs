extern crate yaml_rust;
use yaml_rust::YamlLoader;
use structopt::StructOpt;
use std::io::Read;

mod consts;

#[derive(StructOpt)]
struct Cli {
    /// Location where i can find the service files 'Default: /etc/oxirc/'
    #[structopt(short = "s", long = "svc-directory", default_value = "/etc/oxirc/")]
    svc_directory: String,
}

#[tokio::main]
async fn main() -> std::io::Result<()> {

    let args = Cli::from_args();

    println!{"Working on {}", args.svc_directory};
    
    let config_file_path = format!("{}{}",args.svc_directory,consts::OXIRC_FILE);

    println!("Loading {}", config_file_path);
    
    let mut config_file = match std::fs::File::open(&config_file_path) {
        Ok(r) => r,
        Err(e) => {
            println!("Error opening: {}", &config_file_path);
            return Err(e)
        },
    };

    let mut config_contents = String::new();
    config_file.read_to_string(&mut config_contents).expect("Unable to load data");

    let config_data = &YamlLoader::load_from_str(&config_contents).expect("Error parsing Yaml")[0];

    println!("{}",config_data["oxircApiVersion"].as_str().unwrap());


    loop
    {
    
        std::thread::sleep(std::time::Duration::from_millis(500));
    }
}
       