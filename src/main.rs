use std::env;
use std::process;

use rust_project::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    // print the error message and exit the program if the arguments are not valid
    let config: Config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    if let Err(e) = rust_project::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}

