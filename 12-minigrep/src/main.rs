use std::env;
use std::process;

use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    let config = Config::build(env::args().skip(1)).unwrap_or_else(|error| {
        eprintln!("Problem parsing arguments: {error}");
        process::exit(1);
    });

    if let Err(e) = config.run() {
        eprintln!("Application error: {e}");
        process::exit(1);
    };
}
