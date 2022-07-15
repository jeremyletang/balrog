use crate::cmd::network;
use crate::errors::Error;
use crate::paths;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use url::Url;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub name: String,
    pub chain_id: String,
    pub block_explorer: Url,
    pub primary_rpc: Url,
    pub secondary_rpc: Url,
}

fn save(home: &str, cfg: &Config) -> Result<PathBuf, Error> {
    let path = paths::configuration_file(home);
    let serialized = serde_json::to_string(cfg).unwrap();
    let mut f = File::create(&path)?;
    write!(f, "{}", serialized)?;
    return Ok(path);
}

pub fn load(home: &str, network: &str) -> Result<Config, Error> {
    let mut path = paths::networks_directory(home).join(network);
    path.set_extension("json");
    let contents = std::fs::read_to_string(path)?;
    let ks = serde_json::from_str(&contents)?;
    return Ok(ks);
}
