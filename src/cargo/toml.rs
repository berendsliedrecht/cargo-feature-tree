use core::fmt;

use serde::{Deserialize, Serialize};
use toml::{value::Map, Value};

use super::{features::CargoFeatures, package::CargoPackage, workspace::CargoWorkspace};

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct CargoToml {
    package: Option<CargoPackage>,
    workspace: Option<CargoWorkspace>,
    features: Option<Map<String, Value>>,
    // dependencies: Option<Map<String, Value>>,
}

impl CargoToml {
    pub fn features(&self) -> Option<CargoFeatures> {
        self.features.as_ref().map(CargoFeatures::from_map)
    }
}

impl fmt::Display for CargoToml {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(package) = &self.package {
            let _ = writeln!(f, "=== Package ===");
            let _ = writeln!(f, "{}", package);
        }

        if let Some(workspace) = &self.workspace {
            let _ = writeln!(f, "=== Workspace ===");
            let _ = writeln!(f, "{}", workspace);
        }

        if let Some(features) = self.features() {
            let _ = writeln!(f, "=== Features ===");
            let _ = writeln!(f, "{}", features);
        };

        Ok(())
    }
}
