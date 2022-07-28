use crate::error::{Error, Result};
use cargo_toml::Manifest;
use std::path::Path;

pub struct Parse;

impl Parse {
    fn from_bytes(b: &[u8]) -> Result<Manifest> {
        Manifest::from_slice(b).map_err(|e| Error::UnableToParse(e.to_string()))
    }

    pub fn from_dir(p: impl AsRef<Path>) -> Result<Manifest> {
        let mut p_buf = p.as_ref().to_path_buf();

        if p_buf.is_dir() {
            p_buf.push("Cargo.toml");
        }

        Self::from_file(p_buf)
    }

    fn from_file(p: impl AsRef<Path>) -> Result<Manifest> {
        let bytes = std::fs::read(p).map_err(|_| Error::ManifestNotFound)?;
        Self::from_bytes(&bytes)
    }
}
