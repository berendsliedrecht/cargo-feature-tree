use cargo_toml::Manifest;
use std::{collections::BTreeMap, env, path::PathBuf, process::exit};
use termtree::Tree;

fn err(msg: &str) -> ! {
    eprintln!("{msg}");
    exit(1)
}

fn main() {
    // Get args and skip own name
    let mut args = env::args().skip(1);

    // Can only be ran as cargo subcommand, or in debug
    if !cfg!(debug_assertions) && args.next() != Some(String::from("feature-tree")) {
        err("Can only run feature-tree as a cargo subcommand")
    };

    // Get the path
    let path = args.next().unwrap_or_else(|| String::from("."));
    let mut path = PathBuf::from(path);

    // Add `Cargo.toml` if it is a directory
    if path.is_dir() {
        path.push("Cargo.toml");
    }

    // Extract the manifest or error out
    let mut manifest = match Manifest::from_path(path) {
        Ok(m) => m,
        Err(e) => err(e.to_string().as_str()),
    };
    // Get the features
    let features = &mut manifest.features;

    // Get the name or unnamed if it is a workspace
    let name = manifest.package.map_or("unnamed".to_owned(), |p| p.name);

    let top_level_feature_keys: Vec<String> = args
        .next()
        .map_or_else(|| features.keys().map(Clone::clone).collect(), |f| vec![f]);

    // Get the features
    let mut leaves = vec![];
    for feature in top_level_feature_keys {
        let maybe_tree = get_nested_part(&feature, features);
        if let Some(t) = maybe_tree {
            leaves.push(t);
        }
    }

    // Create the end tree
    let tree = Tree::new(name).with_leaves(leaves);

    // And display!
    println!("{tree}");
}

fn get_nested_part(key: &str, features: &BTreeMap<String, Vec<String>>) -> Option<Tree<String>> {
    let mut leaves = vec![];
    for feature in features.get(key)? {
        let maybe_tree = get_nested_part(feature, features);
        if let Some(t) = maybe_tree {
            leaves.push(t);
        }
    }

    Some(Tree::new(key.to_owned()).with_leaves(leaves))
}
