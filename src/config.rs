use serde::{Deserialize, Serialize};
use std::{fs, io::ErrorKind, path::Path};

#[derive(Serialize, Deserialize)]
pub struct ResourceConfig {
    pub(crate) name: String,
    pub(crate) path: String,
}

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub(crate) resources: Vec<ResourceConfig>,
}

pub fn parse_config_file(file_path: &str) -> Config {
    let contents = fs::read_to_string(file_path).unwrap_or_else(|_| {
        log::warn!("Config file not found, creating one");
        let _config = Config {
            resources: Vec::new(),
        };
        write_config_file(file_path, &_config);
        fs::read_to_string(file_path).expect("Failed to read config file")
    });
    let config: Config = serde_json::from_str(&contents).expect("Failed to parse config file");

    config
}

pub fn write_config_file(file_path: &str, config: &Config) {
    let contents = serde_json::to_string_pretty(config).expect("Failed to serialize config");
    fs::write(file_path, contents.clone()).unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            log::warn!("Directory not found, creating one");
            let dir_path = Path::new(file_path).parent().unwrap();
            fs::create_dir_all(dir_path).expect("Failed to create directory");
            fs::write(file_path, contents).expect("Failed to write config file");
        } else {
            panic!("Failed to write config file");
        }
    });
}

pub fn get_default_config_path() -> String {
    let home_dir = dirs::home_dir().expect("Failed to get home directory");
    let config_dir = home_dir.join(".linker");
    let config_file = config_dir.join("config.json");
    log::info!("Config file path: {}", config_file.to_str().unwrap());
    config_file.to_str().unwrap().to_string()
}
