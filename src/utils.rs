use rand::distributions::Alphanumeric;
use rand::Rng;
pub fn get_default_config_path() -> String {
    let home_dir = dirs::home_dir()
        .expect("Failed to get home directory")
        .join(".linker")
        .join("config.json");
    home_dir.to_str().unwrap().to_string()
}

pub(crate) fn generate_random_string(length: usize) -> String {
    let s: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(length)
        .map(char::from)
        .collect();
    s
}

pub fn create_soft_link(src: &str, dst: &str) {
    let src = std::path::Path::new(src);
    let dst = std::path::Path::new(dst);
    log::info!("Creating soft link: {} -> {}", src.display(), dst.display());

    let dir_path = std::path::Path::new(&dst).parent().unwrap();
    if !dir_path.exists() {
        std::fs::create_dir_all(dir_path).expect("Failed to create directory");
    }

    // platform check
    #[cfg(target_os = "windows")]
    if !dst.exists() {
        std::os::windows::fs::symlink_file(src, dst).expect("Failed to create soft link");
    }

    #[cfg(target_os = "macos")]
    if !dst.exists() {
        std::os::unix::fs::symlink(src, dst).expect("Failed to create soft link");
    }

    #[cfg(target_os = "linux")]
    if !dst.exists() {
        std::os::unix::fs::symlink(src, dst).expect("Failed to create soft link");
    }
}

pub fn remove_soft_link(dst: &str) {
    let dst = std::path::Path::new(dst);
    if dst.exists() {
        std::fs::remove_file(dst).expect("Failed to remove soft link");
    }
}
