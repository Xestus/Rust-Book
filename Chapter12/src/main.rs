use std::fs::File;
use std::io::prelude::*; // glob import -> Bring everything that is available in `...::prelude`
use std::error::Error;

extern crate min;
use min::Config;

fn main() {
    let args: Vec<String> = std::env::args().collect();
        // env -> OS execution environment || args -> returns argument your program runs with.

    let conf = Config::parse(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        std::process::exit(1);
    });

    if let Err(e) = min::run(conf) { // No unwrap... because it doesn't return a value we want to use.
        eprintln!("Application error: {}", e);
        std::process::exit(1);
    }
}

