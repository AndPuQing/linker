/// This module contains the implementation of the configuration functionality for the linker.
/// It provides functions for parsing and writing the configuration file, as well as retrieving the default configuration path.
/// The configuration file is in JSON format and contains information about the resources used by the linker.
use serde::{Deserialize, Serialize};
use std::{fs, io::ErrorKind, path::Path};

use super::utils;

/// Represents a resource configuration.
#[derive(Clone, Serialize, Deserialize)]
pub struct ResourcePair {
    pub(crate) name: String,
    pub(crate) path: String,
}

/// Represents the overall configuration.
#[derive(Serialize, Deserialize)]
pub struct Config {
    pub(crate) resources: Vec<ResourcePair>,
    pub(crate) links: Vec<LinkerLog>,
}
/// Represents a linker log.
#[derive(Serialize, Deserialize)]
pub struct LinkerLog {
    pub(crate) dir: String,
    pub(crate) link: Vec<ResourcePair>,
}

impl Config {
    pub fn new() -> Self {
        let default_path = utils::get_default_config_path();
        let contents = fs::read_to_string(default_path);
        let config: Config = match contents {
            Ok(contents) => serde_json::from_str(&contents).expect("Failed to parse config file"),
            Err(_) => {
                log::warn!("Config file not found, creating one");
                let config = Config {
                    resources: Vec::new(),
                    links: Vec::new(),
                };
                config.save();
                config
            }
        };
        config
    }

    pub fn get_resource(&self, name: &str) -> Option<&ResourcePair> {
        self.resources
            .iter()
            .find(|&resource| resource.name == name)
    }

    pub fn remove_link(&mut self, dir: &str) {
        for (index, link) in self.links.iter().enumerate() {
            if link.dir == dir {
                log::warn!("removing soft link for {}", dir);
                for resource in link.link.iter() {
                    let path = Path::new(dir).join(resource.name.clone());
                    log::warn!("remove soft link: {}", path.to_str().unwrap());
                    utils::remove_soft_link(path.to_str().unwrap());
                }
                self.links.remove(index);
                break;
            }
        }
        self.save();
    }

    pub fn add_link(&mut self, dir: &str, links: toml::map::Map<std::string::String, toml::Value>) {
        let mut link = LinkerLog {
            dir: dir.to_string(),
            link: Vec::new(),
        };

        for (dst, name_orpath) in links.iter() {
            let items = name_orpath.as_table().unwrap();
            let resource = self.get_resource(dst);
            match resource {
                Some(resource) => {
                    if items.contains_key("instance") {
                        let instance = items.get("instance").unwrap().as_bool().unwrap();
                        if instance {
                            let instance_name =
                                resource.name.to_string() + "-" + &utils::generate_random_string(9);
                            log::info!("Creating instance: {}", instance_name);
                            let path = Path::new(&resource.path).join(instance_name);
                            std::fs::create_dir_all(&path).expect("Failed to create directory");
                            let path = path.to_str().unwrap();
                            utils::create_soft_link(path, dst);
                            link.link.push(ResourcePair {
                                name: dst.to_string(),
                                path: path.to_string(),
                            });
                        } else {
                            utils::create_soft_link(&resource.path, dst);
                            link.link.push(ResourcePair {
                                name: dst.to_string(),
                                path: resource.path.to_string(),
                            });
                        }
                    }
                }
                None => {
                    for (dst_, res) in items.iter() {
                        let res = res.as_str().unwrap_or_else(|| {
                            log::error!("Cannot parse resource path: {}", res.to_string());
                            panic!("Cannot parse resource path: {}, maybe you should add {} resource first", res, dst);
                         });
                        let resource = self.get_resource(res);
                        match resource {
                            Some(resource) => {
                                let path = Path::new(dir).join(dst).join(dst_);
                                let path = path.to_str().unwrap();
                                utils::create_soft_link(&resource.path, path);
                                link.link.push(ResourcePair {
                                    name: path.to_string(),
                                    path: resource.path.to_string(),
                                });
                            }
                            None => {
                                let path = Path::new(res);
                                if path.exists() {
                                    utils::create_soft_link(res, dst_);
                                    link.link.push(ResourcePair {
                                        name: dst_.to_string(),
                                        path: res.to_string(),
                                    });
                                } else {
                                    log::error!("Resource {} not found", res);
                                    println!("Resource {} not found", res);
                                }
                            }
                        }
                    }
                }
            }
        }
        self.links.push(link);
        self.save();
    }

    pub fn add_resource(&mut self, name: &str, path: &String) {
        let mut updated = false;

        for resource in self.resources.iter_mut() {
            if resource.name == name {
                updated = true;
                println!("Resource {}: {} --> {}", name, resource.path, path);
                resource.path = path.to_string();
                break;
            }
        }

        if !updated {
            let resource = ResourcePair {
                name: name.to_string(),
                path: path.to_string(),
            };
            self.resources.push(resource);
        }
        self.save();
    }

    pub fn remove_resource(&mut self, name: Option<&str>, all: bool) {
        if all {
            log::info!("Trying to remove all resources");
            self.resources.clear();
            println!("All resources removed");
        } else {
            match name {
                Some(name) => {
                    for (index, resource) in self.resources.iter().enumerate() {
                        if resource.name == name {
                            log::info!("Trying to remove resource {}", name);
                            self.resources.remove(index);
                            println!("Resource {} removed", name);
                            break;
                        }
                    }
                }
                None => {
                    log::error!("Please specify a resource name");
                    println!("Please specify a resource name");
                }
            }
        }
        self.save();
    }

    pub fn save(&self) {
        let default_path = utils::get_default_config_path();
        let contents = serde_json::to_string_pretty(&self).expect("Failed to serialize config");
        fs::write(default_path.clone(), contents.clone()).unwrap_or_else(|error| {
            if error.kind() == ErrorKind::NotFound {
                log::warn!("Directory not found, creating one");
                let dir_path = Path::new(&default_path).parent().unwrap();
                fs::create_dir_all(dir_path).expect("Failed to create directory");
                fs::write(default_path, contents).expect("Failed to write config file");
            } else {
                panic!("Failed to write config file");
            }
        });
    }
}
