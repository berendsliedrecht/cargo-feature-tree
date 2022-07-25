use error::Result;
use parse::Parse;
use std::env;

mod cargo;
mod error;
mod parse;

fn run() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let default_path = String::from("./Cargo.toml");
    let path = &args.get(2).unwrap_or(&default_path);
    let cargo_toml = Parse::from_file(path)?;
    println!("{}", cargo_toml);
    Ok(())
}

fn main() {
    match run() {
        Ok(()) => (),
        // Red and bold
        Err(e) => eprintln!("\x1b[31m\x1b[1m[Error]\x1b[0m: {}", e),
    }
}
