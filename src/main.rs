use crate::{error::Error, features::CargoFeatures};
use parse::Parse;
use std::{env, process::exit};

mod error;
mod features;
mod formatter;
mod parse;

fn main() {
    let mut args = env::args().skip(1);

    if args.next() != Some(String::from("feature-tree")) {
        eprintln!("[ERROR]: {}", Error::OnlyRunAsSubcommand);
        exit(1)
    };

    let path = match args.next() {
        Some(p) => p,
        None => String::from("."),
    };

    let cargo_toml = Parse::from_dir(path);

    match cargo_toml {
        Ok(t) => CargoFeatures(t.features).display(),
        Err(e) => eprintln!("[ERROR]: {}", e),
    }
}
