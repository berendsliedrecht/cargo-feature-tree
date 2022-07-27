use core::fmt;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CargoWorkspace {
    members: Vec<String>,
}

impl fmt::Display for CargoWorkspace {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for member in &self.members {
            let _ = writeln!(f, "  - {}", member);
        }

        Ok(())
    }
}
