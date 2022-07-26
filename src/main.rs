use colored::Colorize;
use error::Result;
use parse::Parse;
use std::env;

mod cargo;
mod error;
mod formatter;
mod parse;

fn run() -> Result<impl std::fmt::Display> {
    let path = env::args().nth(2).unwrap_or_else(|| String::from("."));
    let cargo_toml = Parse::from_dir(path)?;

    Ok(cargo_toml)
}

fn main() {
    match run() {
        Ok(r) => print!("{}", r),
        Err(e) => eprintln!("{}: {}", "[ERROR]".red().bold(), e),
    }
}
