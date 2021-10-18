use std::error::Error as StdError;
use std::fmt;
use std::io::Error as IoError;

#[derive(Debug)]
pub enum Error {
    RootPathIsNotADir,
    IoError(IoError),
    // TomlError(toml::de::Error),
    // JsonError(serde_json::error::Error),
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
            RootPathIsNotADir => "home path is not a directory".into(),
            IoError(err) => err.to_string(),
            // TomlError(err) => err.to_string(),
            // JsonError(err) => err.to_string(),
        }
    }
}
