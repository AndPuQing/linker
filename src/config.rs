/// This module contains the implementation of the configuration functionality for the linker.
/// It provides functions for parsing and writing the configuration file, as well as retrieving the default configuration path.
/// The configuration file is in JSON format and contains information about the resources used by the linker.
use serde::{Deserialize, Serialize};
use std::{fs, io::ErrorKind, path::Path};

/// Represents a resource configuration.
#[derive(Serialize, Deserialize)]
pub struct ResourceConfig {
    pub(crate) name: String,
    pub(crate) path: String,
}

/// Represents the overall configuration.
#[derive(Serialize, Deserialize)]
pub struct Config {
    pub(crate) resources: Vec<ResourceConfig>,
}

/// Parses the configuration file at the specified file path and returns the parsed configuration.
/// If the file does not exist, a new configuration file will be created with default values.
/// If the file cannot be read or parsed, an error will be returned.
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

/// Writes the configuration to the specified file path.
/// If the directory or file does not exist, they will be created.
/// If the file cannot be written, an error will be returned.
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

/// Returns the default configuration file path.
/// The default path is in the user's home directory under the ".linker" directory.
pub fn get_default_config_path() -> String {
    let home_dir = dirs::home_dir().expect("Failed to get home directory");
    let config_dir = home_dir.join(".linker");
    let config_file = config_dir.join("config.json");
    log::info!("Config file path: {}", config_file.to_str().unwrap());
    config_file.to_str().unwrap().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_write_config_file() {
        let _config = parse_config_file("config.json");
        let _resource = ResourceConfig {
            name: "test".to_string(),
            path: "test".to_string(),
        };
        let _config = Config {
            resources: vec![_resource],
        };
        write_config_file("config.json", &_config);
        let _config = parse_config_file("config.json");
        assert_eq!(_config.resources.len(), 1);
        fs::remove_file("config.json").unwrap();
    }
}
