use std::error::Error as StdError;
use std::fmt;
use std::io;

#[derive(Debug)]
pub enum Error {
    InvalidIndex,
    InvalidSalt,
    InvalidHexCypherText,
    Slip10Error(slip10::Error),
    Bip39Error(bip39::Error),
    IoError(io::Error),
    JsonError(serde_json::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "configuration error: {}", self.desc())
    }
}

impl From<serde_json::Error> for Error {
    fn from(error: serde_json::Error) -> Self {
        Error::JsonError(error)
    }
}

impl From<io::Error> for Error {
    fn from(error: io::Error) -> Self {
        Error::IoError(error)
    }
}

impl From<bip39::Error> for Error {
    fn from(error: bip39::Error) -> Self {
        Error::Bip39Error(error)
    }
}

impl From<slip10::Error> for Error {
    fn from(error: slip10::Error) -> Self {
        Error::Slip10Error(error)
    }
}

impl StdError for Error {}

impl Error {
    pub fn desc(&self) -> String {
        use Error::*;
        match self {
            InvalidIndex => "index must be > 0".into(),
            InvalidSalt => "salt must be in valid hex format".into(),
            InvalidHexCypherText => "cypher_test must be in valid hex format".into(),
            Slip10Error(e) => format!("slip10 error: {}", e.desc()),
            Bip39Error(e) => format!("bip39 error: {}", e),
            IoError(e) => format!("IO error: {}", e),
            JsonError(e) => format!("json error: {}", e),
        }
    }
}
