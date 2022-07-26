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

// TODO: refactor and should be recursive
impl fmt::Display for CargoFeatures {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let indent = "  ";
        let list_marker = "-";
        let arr = &self.0;
        let features = self.get_features();
        arr.iter().for_each(|(feature, other_features)| {
            if feature == "default" {
                let _ = writeln!(f, "{}", feature.cyan());
            } else {
                let _ = writeln!(f, "{}", feature);
            }
            other_features.iter().for_each(|other_feature| {
                let _ = writeln!(f, "{}{} {}", indent, list_marker, other_feature);
                if features.contains(other_feature) {
                    if let Some(sub_features) = self.get_other_features_by_feature(other_feature) {
                        sub_features.iter().for_each(|s| {
                            let _ = writeln!(f, "{}{}{} {}", indent, indent, list_marker, s);
                        });
                    }
                }
            });
        });

        Ok(())
    }
}
