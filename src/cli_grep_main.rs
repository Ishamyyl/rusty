use std::{env, process};

use rusty::Config;

fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = rusty::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
