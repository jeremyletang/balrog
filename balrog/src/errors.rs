use super::keystore;
use std::error::Error as StdError;
use std::fmt;
use std::io;

#[derive(Debug)]
pub enum Error {
    HomePathIsNotADir,
    PassphrasesDoesNotMatch,
    IoError(io::Error),
    KeystoreError(keystore::Error),
    // TomlError(toml::de::Error),
    // JsonError(serde_json::error::Error)
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "configuration error: {}", self.desc())
    }
}

impl From<io::Error> for Error {
    fn from(error: io::Error) -> Self {
        Error::IoError(error)
    }
}

impl From<keystore::Error> for Error {
    fn from(error: keystore::Error) -> Self {
        Error::KeystoreError(error)
    }
}

impl StdError for Error {}

impl Error {
    pub fn desc(&self) -> String {
        use Error::*;
        match self {
            HomePathIsNotADir => "home path is not a directory".into(),
            PassphrasesDoesNotMatch => "passphrases does not match".into(),
            IoError(err) => format!("IO error: {}", err.to_string()),
            KeystoreError(err) => format!("keystore error: {}", err.to_string()),
            // TomlError(err) => err.to_string(),
            // JsonError(err) => err.to_string(),
        }
    }
}
