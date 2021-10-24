use crate::errors::Error;
use crate::paths;
use serde::{Deserialize, Serialize};
use walkdir::WalkDir;

#[derive(Serialize, Deserialize)]
pub struct Accounts {
    accounts: Vec<String>,
}

pub fn accounts(home: &str) -> Result<Accounts, Error> {
    let accs = paths::accounts_directory(home);
    if !accs.exists() || !accs.is_dir() {
        return Err(Error::HomePathIsNotADir);
    }

    let mut accounts = vec![];

    for entry in WalkDir::new(accs).into_iter().filter_map(|e| {
        if let Ok(eok) = e {
            if (eok.path().display().to_string()).ends_with(".json") {
                return Some(eok);
            }
            // return eok.to_string().ends_with(".json");
        }
        return None;
    }) {
        accounts.push(
            entry
                .path()
                .file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .to_string()
                .trim_end_matches(".json")
                .to_string(),
        );
    }

    return Ok(Accounts { accounts });
}
