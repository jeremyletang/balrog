use crate::paths;
use crate::{config, errors::Error};
use serde::{Deserialize, Serialize};
use std::path::Path;

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
    let accounts_path = paths::accounts_directory(home);
    let networks_path = paths::networks_directory(home);
    let configuration_path = paths::configuration_file(home);
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
        config::save(
            home,
            &config::Config {
                account: "".into(),
                network: "".into(),
            },
        )?;
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
