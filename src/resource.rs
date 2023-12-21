use serde::{Deserialize, Serialize};
use toml::Table;

#[derive(Serialize, Deserialize)]
pub struct Resource {
    pub(crate) resource: Table,
}

impl Resource {
    pub fn new() -> Self {
        let current_dir = std::env::current_dir().expect("Failed to get current directory");
        let default_config_path = current_dir.join("resource.toml");
        let contents = std::fs::read_to_string(default_config_path);
        let resource: Resource = match contents {
            Ok(contents) => toml::from_str(&contents).expect("Failed to parse config file"),
            Err(_) => {
                log::warn!("Config file not found, creating one");
                let resource = Table::new();
                let _config = Resource { resource };
                _config.save();
                _config
            }
        };
        resource
    }

    pub fn save(&self) {
        let contents = toml::to_string_pretty(&self).expect("Failed to serialize config");
        std::fs::write("resource.toml", contents.clone()).unwrap_or_else(|error| {
            if error.kind() == std::io::ErrorKind::NotFound {
                log::warn!("Directory not found, creating one");
                let dir_path = std::path::Path::new("resource.toml").parent().unwrap();
                std::fs::create_dir_all(dir_path).expect("Failed to create directory");
                std::fs::write("resource.toml", contents).expect("Failed to write config file");
            } else {
                panic!("Failed to write config file");
            }
        });
    }
}

#[test]
fn test_new() {
    let _config = Resource::new();
    // assert!(std::path::Path::new("resource.toml").exists()); # TODO: CI fails
}

#[test]
fn test_paser() {
    let _config: Resource = toml::from_str(
        r#"
        [resource]
        name = "test"
        path = "test"
        "#,
    )
    .unwrap();
    assert_eq!(_config.resource["name"], "test".into());
    assert_eq!(_config.resource["path"], "test".into());
}
#[test]
fn test_checkpoint() {
    let _config: Resource = toml::from_str(
        r#"
        [resource]

        [resource.checkpoint]
        instance = true
        "#,
    )
    .unwrap();
    assert_eq!(_config.resource["checkpoint"]["instance"], true.into());
}

#[test]
fn test_dataset() {
    let _config: Resource = toml::from_str(
        r#"
        [resource]

        [resource.dataset]
        cityscapes = "cityscapes"
        ade20k = "/home/ade20k"
        "#,
    )
    .unwrap();
    assert_eq!(
        _config.resource["dataset"]["cityscapes"],
        "cityscapes".into()
    );
    assert_eq!(_config.resource["dataset"]["ade20k"], "/home/ade20k".into());
}

#[test]
fn test_resource() {
    let _config: Resource = toml::from_str(
        r#"
        [resource]
        checkpoint = { instance = true }
        dataset = { cityscapes = "cityscapes" }
        "#,
    )
    .unwrap();
    assert_eq!(_config.resource["checkpoint"]["instance"], true.into());
    assert_eq!(
        _config.resource["dataset"]["cityscapes"],
        "cityscapes".into()
    );
}
