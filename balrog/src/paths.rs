use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

const ACCOUNTS_PATH: &str = "accounts";
const NETWORKS_PATH: &str = "networks";
const CONFIG_PATH: &str = "config.toml";

#[derive(Serialize, Deserialize)]
pub struct Paths {
    pub home: String,
    pub configuration: String,
    pub accounts: String,
    pub networks: String,
}

pub fn info(home: &str) -> Paths {
    return Paths {
        home: Path::new(home).to_str().unwrap().to_string(),
        configuration: configuration_file(home).to_str().unwrap().to_string(),
        accounts: accounts_directory(home).to_str().unwrap().to_string(),
        networks: networks_directory(home).to_str().unwrap().to_string(),
    };
}

pub fn accounts_directory(home: &str) -> PathBuf {
    return Path::new(home).join(Path::new(ACCOUNTS_PATH));
}

pub fn networks_directory(home: &str) -> PathBuf {
    return Path::new(home).join(Path::new(NETWORKS_PATH));
}

pub fn configuration_file(home: &str) -> PathBuf {
    return Path::new(home).join(Path::new(CONFIG_PATH));
}
