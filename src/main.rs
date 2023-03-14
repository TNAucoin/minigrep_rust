use std::env;
use std::process;

use minigrep::Config;

fn main() {
    // Collect arguments into a Vec
    let args: Vec<String> = env::args().collect();

    // unwrap if no error, otherwise handle the error.
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    // use if let here, as run doesn't return a value for unwrap.
    if let Err(e) = minigrep::run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }
}
