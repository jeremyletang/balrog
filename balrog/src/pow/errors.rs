use std::error::Error as StdError;
use std::fmt;

#[derive(Debug)]
pub enum Error {
    InvalidDifficulty,
    EmptyTxId,
    InvalidBlockHash,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "keystore error: {}", self.desc())
    }
}

impl StdError for Error {}

impl Error {
    pub fn desc(&self) -> String {
        use Error::*;
        match self {
            InvalidDifficulty => "invalid difficulty".into(),
            EmptyTxId => "empty transaction id".into(),
            InvalidBlockHash => "invalid block hash".into(),
        }
    }
}
