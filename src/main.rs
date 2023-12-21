use std::path;

use clap::{Args, Parser, Subcommand};
use clap_verbosity_flag::Verbosity;
mod config;

#[derive(Parser, Debug)]
#[command(author, version, about, name = "DL Resource Manager")]

struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    #[command(flatten)]
    verbose: Verbosity,
}
#[derive(Subcommand, Debug)]
enum Commands {
    /// list soft links on current directory
    /// or on the path provided
    List(ListArgs),

    /// add a new resource
    /// or update an existing one
    Add(AddArgs),

    /// remove a resource
    /// or all resources
    Remove(RemoveArgs),
}

#[derive(Args, Debug)]
struct ListArgs {
    /// path to list soft links
    path: Option<String>,
}

#[derive(Args, Debug)]
struct AddArgs {
    #[arg(required = true)]
    /// name of the resource
    name: String,

    #[arg(required = true)]
    /// path to the resource
    path: path::PathBuf,
}

#[derive(Args, Debug)]
struct RemoveArgs {
    /// name of the resource
    name: Option<String>,

    /// remove all resources
    #[arg(short, long)]
    all: bool,
}

fn main() {
    let cli = Cli::parse();
    env_logger::Builder::new()
        .filter_level(cli.verbose.log_level_filter())
        .init();
    let _default_config_path = config::get_default_config_path();
    match &cli.command {
        Some(Commands::List(name)) => match name.path {
            Some(ref _name) => {
                println!("{}", _name);
            }
            None => {
                log::info!("No path provided, listing current directory");
            }
        },
        Some(Commands::Add(name)) => {
            log::info!(
                "Trying to add resource {} at {}",
                name.name,
                name.path.display()
            );
            let mut _config = config::parse_config_file(&_default_config_path);
            let mut updated = false;

            for resource in _config.resources.iter_mut() {
                if resource.name == name.name {
                    updated = true;
                    println!(
                        "Resource {}: {} --> {}",
                        name.name,
                        resource.path,
                        name.path.to_str().unwrap()
                    );
                    resource.path = name.path.to_str().unwrap().to_string();
                    break;
                }
            }

            if !updated {
                let _resource = config::ResourceConfig {
                    name: name.name.to_string(),
                    path: name.path.to_str().unwrap().to_string(),
                };
                _config.resources.push(_resource);
            }

            config::write_config_file(&_default_config_path, &_config);
        }
        Some(Commands::Remove(name)) => match name.name {
            Some(ref _name) => {
                log::info!("Trying to remove resource {}", _name);
                let mut _config = config::parse_config_file(&_default_config_path);
                let mut updated = false;

                for (index, resource) in _config.resources.iter().enumerate() {
                    if resource.name == *_name {
                        updated = true;
                        println!("Resource {} removed", _name);
                        _config.resources.remove(index);
                        break;
                    }
                }

                if !updated {
                    println!("Resource {} not found", _name);
                }

                config::write_config_file(&_default_config_path, &_config);
            }
            None => {
                if name.all {
                    log::info!("Trying to remove all resources");
                    let _config = config::Config {
                        resources: Vec::new(),
                    };
                    config::write_config_file(&_default_config_path, &_config);
                } else {
                    log::info!("No resource name provided, nothing to remove");
                }
            }
        },
        None => {}
    }
}
