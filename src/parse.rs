use crate::error::{Error, Result};
use cargo_toml::Manifest;
use std::path::Path;

pub struct Parse;

impl Parse {
    fn from_bytes(b: &[u8]) -> Result<Manifest> {
        Manifest::from_slice(b).map_err(|e| Error::UnableToParse(e.to_string()))
    }

    pub fn from_dir(p: impl AsRef<Path>) -> Result<Manifest> {
        let mut p = p.as_ref().to_path_buf();

        if p.is_dir() {
            p.push("Cargo.toml");
        }

        Parse::from_file(p)
    }

    fn from_file(p: impl AsRef<Path>) -> Result<Manifest> {
        let bytes = std::fs::read(p).map_err(|_| Error::ManifestNotFound)?;
        Parse::from_bytes(&bytes)
    }
}
