use core::fmt;

use serde::{Deserialize, Serialize};
use toml::{value::Map, Value};

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct CargoFeatures(Vec<(String, Vec<String>)>);

impl CargoFeatures {
    pub fn from_map(map: &Map<String, Value>) -> Self {
        let mapped_map = map
            .into_iter()
            .map(|(name, value)| {
                let arr = value
                    .as_array()
                    .unwrap()
                    .iter()
                    .map(|v| v.as_str().unwrap().to_owned())
                    .collect();
                (name.to_owned(), arr)
            })
            .collect();

        Self(mapped_map)
    }

    pub fn get_other_features_by_feature(&self, name: &str) -> Option<Vec<String>> {
        self.0
            .iter()
            .find(|(k, _)| k == name)
            .map(|(_, v)| v.to_owned())
    }

    pub fn get_features(&self) -> Vec<String> {
        self.0.iter().map(|(n, _)| n.to_owned()).collect()
    }
}

impl fmt::Display for CargoFeatures {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let arr = &self.0;
        let names = self.get_features();
        arr.iter().for_each(|(name, features)| {
            let _ = writeln!(f, "{}", name);
            features.iter().for_each(|feature| {
                let _ = writeln!(f, "  - {}", feature);
                if names.contains(feature) {
                    let sub_features = self.get_other_features_by_feature(feature).unwrap();
                    sub_features.iter().for_each(|s| {
                        let _ = writeln!(f, "    - {}", s);
                    });
                }
            });
        });

        Ok(())
    }
}
