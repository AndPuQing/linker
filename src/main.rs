use clap::{Args, Parser, Subcommand};
use clap_verbosity_flag::Verbosity;
mod config;
mod resource;
mod utils;

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
    /// LinkStart links resources to the current directory
    Link,

    /// list soft links on current directory
    /// or on the path provided
    List,

    /// add a new resource
    /// or update an existing one
    Add(AddArgs),

    /// remove a resource
    /// or all resources
    Remove(RemoveArgs),

    /// initialize the configuration file
    Init,
}

#[derive(Args, Debug)]
struct AddArgs {
    #[arg(required = true)]
    /// name of the resource
    name: String,

    #[arg(required = true)]
    /// path to the resource
    path: String,
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
    match &cli.command {
        Some(Commands::Link) => {
            let resource = resource::Resource::new();
            let mut config = config::Config::new();
            let current_dir = std::env::current_dir().expect("Failed to get current directory");
            let current_dir = current_dir.to_str().unwrap().to_string();
            config.remove_link(&current_dir);
            config.add_link(&current_dir, resource.resource);
        }
        Some(Commands::List) => {
            let linker_logs = config::Config::new().links;
            let current_dir = std::env::current_dir().expect("Failed to get current directory");
            let current_dir = current_dir.to_str().unwrap().to_string();
            let mut found = false;
            for linker_log in linker_logs.iter() {
                if linker_log.dir == current_dir {
                    found = true;
                    for resource in linker_log.link.iter() {
                        println!("{} --> {}", resource.name, resource.path);
                    }
                }
            }
            if !found {
                println!("No resource found! May be you should run `linker linkstart`");
            }
        }
        Some(Commands::Add(name)) => {
            let mut config = config::Config::new();
            config.add_resource(&name.name, &name.path);
        }
        Some(Commands::Remove(name)) => match name.name {
            Some(ref _name) => {
                let mut config = config::Config::new();
                config.remove_resource(Some(_name), false);
            }
            None => {
                if name.all {
                    let mut config = config::Config::new();
                    config.remove_resource(None, true);
                }
            }
        },
        Some(Commands::Init) => {
            let resource = resource::Resource::new();
            if !resource.resource.is_empty() {
                log::info!("Resource have {} entries", resource.resource.len());
                println!("File resource.toml exists, nothing to initialize");
            } else {
                println!("File resource.toml created")
            }
        }
        None => {}
    }
}
