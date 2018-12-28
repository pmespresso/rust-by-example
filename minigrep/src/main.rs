 // Rust standard library to enable minigrep to read values of command line arguments
use std::env;

use std::process;

use minigrep;
use minigrep::Config;

fn main() {
    // env::args() returns an Iterator of command line arguments
    // collect() can return many kinds of collections, so we explicitly annotate args to sepficy we want a Vector of Strings
    let args: Vec<String> = env::args().collect(); // we can call collect() on it to turn it into a colection, e.g. a Vector, containing all the elements the iterator produces

    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    };
}
