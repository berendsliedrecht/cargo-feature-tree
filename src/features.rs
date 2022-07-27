use crate::formatter::Formatter;
use core::fmt;
use indexmap::IndexSet;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct CargoFeatures(pub BTreeMap<String, Vec<String>>);

impl<'a> CargoFeatures {
    pub(crate) fn get_other_features_by_feature(
        &self,
        name: impl AsRef<str>,
    ) -> Option<(String, Vec<String>)> {
        self.0
            .get_key_value(name.as_ref())
            .map(|(a, b)| (a.to_owned(), b.to_owned()))
    }

    pub(crate) fn get_other_features_with_depth(
        &self,
        name: impl AsRef<str> + 'a,
        v: &mut IndexSet<(String, usize)>,
        depth: usize,
    ) {
        if let Some((_, other_features)) = self.get_other_features_by_feature(name.as_ref()) {
            other_features.iter().for_each(|feature| {
                v.insert((feature.to_owned(), depth));
                self.get_other_features_with_depth(feature, v, depth + 1);
            });
        }
    }
}

impl fmt::Display for CargoFeatures {
    fn fmt(&self, _: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut v: IndexSet<(String, usize)> = IndexSet::new();

        self.0.iter().for_each(|(name, _)| {
            v.insert((name.to_string(), 0));
            self.get_other_features_with_depth(name, &mut v, 1);
        });

        Formatter::new(v).write();

        Ok(())
    }
}
