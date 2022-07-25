use core::fmt;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct CargoPackage {
    pub name: String,
    pub version: String,
    pub edition: Option<String>,
}

impl fmt::Display for CargoPackage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let _ = writeln!(f, "Name: {}", self.name);
        let _ = writeln!(f, "Version: {}", self.version);

        if let Some(e) = &self.edition {
            let _ = writeln!(f, "Edition: {}", e);
        };

        Ok(())
    }
}
