pub mod configs;

use directories::ProjectDirs;
use ftail::Ftail;
use log::LevelFilter;
use std::fs;
use serde::{Deserialize, Serialize};
use serde_json;
use configs::Configs;
use reqwest::{self};

fn main() {
    match Ftail::new().console(LevelFilter::Info).init() {
        Ok(_) => log::info!("Logger successfully setup"),
        Err(e) => panic!("Logger failed to setup due to {}", e),
    }
    
    let project_directories = ProjectDirs::from("", "Bridgette apps", "Rykeeper").unwrap();
    check_directories(&project_directories);

}

fn get_configs(project_directories: &ProjectDirs) -> Configs {
    let json_file = project_directories.config_local_dir().join("configs.json");
    let contents = match fs::read_to_string(&json_file) {
        Ok(contents) => contents,
        Err(error) => panic!("Could not read from {:?} due to {}", json_file, error)
    };
    serde_json::from_str(&contents).unwrap()
}

fn check_directories(project_directories: &ProjectDirs) -> bool{
    //todo!("Remake this later pls");
    // Ensures that all the necessary directories and files exist
    // Returns false if the configuration file does not exist, true if it does
    let configs_url = "https://raw.githubusercontent.com/Slightly-Deranged-Lunatic/Rykeeper/refs/heads/main/src/configs.json";
    let config_directory = project_directories.config_local_dir();
    if ! fs::exists(&config_directory.join("configs.json")).unwrap() {
        let _ = fs::create_dir_all(&config_directory);
        let default_configs = reqwest::blocking::get(configs_url).unwrap();
        if ! default_configs.status().is_success() {
            panic!("Something went wrong with getting the configuration file, status is {}", default_configs.status())
        }
        let _ = fs::write(config_directory.join("configs.json"), default_configs.text().unwrap());
        return false;
    } else {
        log::info!("Directories and files exist");
        return true;
    }
}