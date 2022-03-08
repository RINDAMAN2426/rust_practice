use std::env;
use std::process;

use chapter12_minigrep::Config;
use chapter12_minigrep as minigrep;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
      eprintln!("Problem parsing arguments: {}", err);
      process::exit(1);
    });

    println!("Searching for \"{}\"", config.query);
    println!("In file \"{}\"", config.filename);

    if let Err(e) = minigrep::run(config) {
      eprintln!("Application error {}", e);

      process::exit(1);
    }
}

