use crate::formatter::Formatter;
use core::fmt;
use indexmap::IndexSet;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct CargoFeatures(pub BTreeMap<String, Vec<String>>);

#[derive(Hash, Eq, PartialEq, Debug)]
pub struct CargoFeature(String, usize);

impl<'a> CargoFeatures {
    pub fn get_other_features_by_feature(
        &self,
        name: impl AsRef<str>,
    ) -> Option<(String, Vec<String>)> {
        self.0
            .get_key_value(name.as_ref())
            .map(|(a, b)| (a.to_owned(), b.to_owned()))
    }

    pub fn get_other_features_with_depth(
        &self,
        name: impl AsRef<str> + 'a,
        v: &mut IndexSet<CargoFeature>,
        depth: usize,
    ) {
        if let Some((_, other_features)) = self.get_other_features_by_feature(name.as_ref()) {
            other_features.iter().for_each(|feature| {
                v.insert(CargoFeature(feature.to_owned(), depth));
                self.get_other_features_with_depth(feature, v, depth + 1);
            });
        }
    }
}

impl fmt::Display for CargoFeatures {
    fn fmt(&self, _: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut v: IndexSet<CargoFeature> = IndexSet::new();

        self.0.iter().for_each(|(name, _)| {
            v.insert(CargoFeature(name.to_string(), 0));
            self.get_other_features_with_depth(name, &mut v, 1);
        });

        Formatter::new(v.iter().map(|feat| (feat.0.as_str(), feat.1)).collect()).write();

        Ok(())
    }
}
