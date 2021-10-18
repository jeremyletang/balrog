use bip39::Error as Bip39Error;
use slip10::Error as Slip10Error;
use std::error::Error as StdError;
use std::fmt;

#[derive(Debug)]
pub enum Error {
    InvalidIndex,
    InvalidSalt,
    InvalidHexCypherText,
    Slip10(Slip10Error),
    Bip39(Bip39Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "configuration error: {}", self.desc())
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
            Slip10(e) => format!("slip10 error: {}", e.desc()),
            Bip39(e) => format!("bip39 error: {}", e),
        }
    }
}
