use core::fmt;

use serde::{Deserialize, Serialize};
use toml::{value::Map, Value};

use super::{features::CargoFeatures, package::CargoPackage};

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct CargoToml {
    pub package: CargoPackage,
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
        let features = self.features().unwrap();
        let _ = writeln!(f, "===== Package =====");
        let _ = writeln!(f, "{}", self.package);
        let _ = writeln!(f, "===== Features =====");
        let _ = writeln!(f, "{}", features);

        Ok(())
    }
}
