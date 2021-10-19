use crate::errors::Error;
use serde::{Deserialize, Serialize};
use std::path::Path;

const ACCOUNTS_PATH: &str = "accounts";
const NETWORKS_PATH: &str = "networks";
const CONFIG_PATH: &str = "config.toml";

#[derive(Serialize, Deserialize, Debug)]
pub struct Paths {
    pub info: Option<String>,
    pub home: String,
    pub accounts: String,
    pub networks: String,
    pub configuration: String,
}

pub fn run(home: &str, _force: bool) -> Result<Paths, Error> {
    let home_path = Path::new(home);
    let accounts_path = home_path.join(Path::new(ACCOUNTS_PATH));
    let networks_path = home_path.join(Path::new(NETWORKS_PATH));
    let configuration_path = home_path.join(Path::new(CONFIG_PATH));
    let paths = vec![home_path, accounts_path.as_path(), networks_path.as_path()];
    let mut info: Option<String> = None;

    if home_path.exists() && home_path.is_dir() {
        // already initialised, set the information message
        info = Some(
            "balrog already initialised, use --force to overwrite existing configuration"
                .to_string(),
        );
    } else if home_path.exists() && !home_path.is_dir() {
        return Err(Error::HomePathIsNotADir);
    } else {
        for path in paths.iter() {
            let _ = ::std::fs::create_dir(&path)?;
        }
    }
    return Ok(Paths {
        info,
        home: home_path
            .to_path_buf()
            .into_os_string()
            .into_string()
            .unwrap(),
        accounts: accounts_path.into_os_string().into_string().unwrap(),
        networks: networks_path.into_os_string().into_string().unwrap(),
        configuration: configuration_path.into_os_string().into_string().unwrap(),
    });
}
