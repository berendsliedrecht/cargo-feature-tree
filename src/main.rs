use crate::features::CargoFeatures;
use parse::Parse;
use std::env;

mod error;
mod features;
mod formatter;
mod parse;

fn main() {
    let path = env::args().nth(2).unwrap_or_else(|| String::from("."));
    let cargo_toml = Parse::from_dir(path);

    match cargo_toml {
        Ok(t) => println!("{}", CargoFeatures(t.features)),
        Err(e) => eprintln!("[ERROR]: {}", e),
    }
}
