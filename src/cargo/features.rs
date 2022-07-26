use colored::Colorize;
use core::fmt;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use toml::{value::Map, Value};

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct CargoFeatures(BTreeMap<String, Vec<String>>);

impl CargoFeatures {
    pub fn from_map(map: &Map<String, Value>) -> Self {
        let mapped_map = map
            .iter()
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
        self.0.get_key_value(name).map(|(_, v)| v.to_owned())
    }

    pub fn get_features(&self) -> Vec<String> {
        self.0.keys().map(|k| k.to_owned()).collect()
    }
}

// TODO: refactor
impl fmt::Display for CargoFeatures {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let arr = &self.0;
        let names = self.get_features();
        arr.iter().for_each(|(name, features)| {
            if name == "default" {
                let _ = writeln!(f, "{}", name.cyan());
            } else {
                let _ = writeln!(f, "{}", name);
            }
            features.iter().for_each(|feature| {
                let _ = writeln!(f, "  - {}", feature);
                if names.contains(feature) {
                    if let Some(sub_features) = self.get_other_features_by_feature(feature) {
                        sub_features.iter().for_each(|s| {
                            let _ = writeln!(f, "    - {}", s);
                        });
                    }
                }
            });
        });

        Ok(())
    }
}
