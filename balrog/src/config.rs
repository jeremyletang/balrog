use crate::errors::Error;
use crate::paths;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub account: String,
    pub network: String,
}

pub fn save(home: &str, cfg: &Config) -> Result<PathBuf, Error> {
    let path = paths::configuration_file(home);
    let serialized = serde_json::to_string(cfg).unwrap();
    let mut f = File::create(&path)?;
    write!(f, "{}", serialized)?;
    return Ok(path);
}

pub fn load(home: &str) -> Result<Config, Error> {
    let path = paths::configuration_file(home);
    let contents = std::fs::read_to_string(path)?;
    let ks = serde_json::from_str(&contents)?;
    return Ok(ks);
}

pub fn set_account(home: &str, value: &str) -> Result<Config, Error> {
    let mut cfg = load(home)?;
    cfg.account = value.into();
    let _ = save(home, &cfg)?;
    return Ok(cfg);
}

pub fn set_network(home: &str, value: &str) -> Result<Config, Error> {
    let mut cfg = load(home)?;
    cfg.network = value.into();
    let _ = save(home, &cfg)?;
    return Ok(cfg);
}
