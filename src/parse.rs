use crate::cargo::toml::CargoToml;
use crate::error::{Error, Result};
use std::path::Path;

pub struct Parse;

impl Parse {
    pub fn from_bytes(b: &[u8]) -> Result<CargoToml> {
        toml::from_slice(b).map_err(|e| Error::UnableToParse(e.to_string()))
    }

    pub fn from_file(p: impl AsRef<Path>) -> Result<CargoToml> {
        let x = std::fs::read(p).map_err(|_| Error::FileNotFound)?;
        Parse::from_bytes(&x)
    }
}
