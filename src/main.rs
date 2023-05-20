use cargo_toml::Manifest;
use std::{collections::BTreeMap, env, path::PathBuf, process::exit};
use termtree::Tree;

fn err(msg: &str) -> ! {
    eprintln!("{}", msg);
    exit(1)
}

fn main() {
    let mut args = env::args().skip(1);

    if !cfg!(debug_assertions) && args.next() != Some(String::from("feature-tree")) {
        err("Can only run feature-tree as a cargo subcommand")
    };

    let path = args.next().unwrap_or_else(|| String::from("."));
    let mut path = PathBuf::from(path);
    if path.is_dir() {
        path.push("Cargo.toml");
    }

    let manifest = match Manifest::from_path(path) {
        Ok(m) => m,
        Err(e) => err(e.to_string().as_str()),
    };
    let features = &manifest.features;
    let name = manifest.package.map_or("unnamed".to_owned(), |p| p.name);

    let mut leaves = vec![];
    for feature in features.keys() {
        let maybe_tree = get_nested_part(feature, features);
        if let Some(t) = maybe_tree {
            leaves.push(t);
        }
    }

    let tree = Tree::new(name).with_leaves(leaves);

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
