use crate::{error::Error, features::CargoFeatures};
use parse::Parse;
use std::{env, process::exit};

mod error;
mod features;
mod parse;
mod tree_formatter;

fn main() {
    let mut args = env::args().skip(1);

    if !cfg!(debug_assertions) && args.next() != Some(String::from("feature-tree")) {
        eprintln!("[ERROR]: {}", Error::OnlyRunAsSubcommand);
        exit(1)
    };

    let path = args.next().map_or_else(|| String::from("."), |p| p);

    let cargo_toml = Parse::from_dir(path);

    match cargo_toml {
        Ok(t) => CargoFeatures(t.features).display(),
        Err(e) => eprintln!("[ERROR]: {e}"),
    }
}
