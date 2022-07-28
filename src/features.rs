use crate::tree_formatter::TreeFormatter;
use core::fmt;
use indexmap::IndexSet;
use serde::{Deserialize, Serialize};
use std::{borrow, collections::BTreeMap};

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct CargoFeatures(pub BTreeMap<String, Vec<String>>);

impl<'a> CargoFeatures {
    fn get_other_features_by_feature(
        &self,
        name: impl AsRef<str>,
    ) -> Option<(String, Vec<String>)> {
        self.0
            .get_key_value(name.as_ref())
            .map(|(a, b)| (a.clone(), b.clone()))
    }

    fn get_other_features_with_depth(
        &self,
        name: impl AsRef<str> + 'a,
        v: &mut IndexSet<(String, usize)>,
        depth: usize,
    ) {
        if let Some((_, other_features)) = self.get_other_features_by_feature(name.as_ref()) {
            for feature in &other_features {
                v.insert((feature.clone(), depth));
                let new_depth = depth.checked_add(1).expect("depth overflow");
                self.get_other_features_with_depth(feature, v, new_depth);
            }
        }
    }

    pub fn display(&self) {
        let mut v: IndexSet<(String, usize)> = IndexSet::new();

        self.0.iter().for_each(|(name, _)| {
            v.insert((name.to_string(), 0));
            self.get_other_features_with_depth(name, &mut v, 1);
        });

        let x: Vec<(String, usize)> = v.iter().map(borrow::ToOwned::to_owned).collect();
        TreeFormatter::new(None).write(x);
    }
}

impl fmt::Display for CargoFeatures {
    fn fmt(&self, _: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.display();
        Ok(())
    }
}
