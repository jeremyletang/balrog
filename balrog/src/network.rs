use crate::errors::Error;
use crate::paths;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::str::FromStr;
use url::Url;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub name: String,
    pub chain_id: String,
    pub block_explorer: Url,
    pub primary_rpc: Url,
    pub secondary_rpc: Url,
}

pub fn save_defaults(home: &str) -> Result<(), Error> {
    let nets: &[Config] = &[
        Config {
            name: "fairground".into(),
            chain_id: "testnet-5f61ab".into(),
            block_explorer: Url::from_str("https://explorer.fairground.wtf").unwrap(),
            primary_rpc: Url::from_str("tcp://n10.testnet.vega.xyz:3007").unwrap(),
            secondary_rpc: Url::from_str("tcp://n09.testnet.vega.xyz:3007").unwrap(),
        },
        Config {
            name: "mainnet".into(),
            chain_id: "vega-mainnet-008".into(),
            block_explorer: Url::from_str("https://explorer.vega.xyz").unwrap(),
            primary_rpc: Url::from_str("tcp://mainnet-observer.ops.vega.xyz:3007").unwrap(),
            secondary_rpc: Url::from_str("tcp://mainnet-observer.ops.vega.xyz:3007").unwrap(),
        },
    ];

    for net in nets.iter() {
        let _ = save(home, &net.name, &net)?;
    }

    return Ok(());
}

fn save(home: &str, network: &str, cfg: &Config) -> Result<PathBuf, Error> {
    let mut path = paths::networks_directory(home).join(network);
    path.set_extension("json");
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
