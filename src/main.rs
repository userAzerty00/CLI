use std::env;
use std::fs;
use std::process;

//This is the main function of the program, it is the entry point of the program
fn main() {
    let args: Vec<String> = env::args().collect();

    // print the error message and exit the program if the arguments are not valid
    let config: Config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    run(config);
}

// This function takes a Config struct as an argument and reads the contents of the file specified in the Config struct, then prints the contents to the console
fn run(config: Config) {
    let contents =
        fs::read_to_string(&config.filename)
            .expect("Something went wrong reading the file");

    println!("With text: \n{}", contents);
}

struct Config {
    query: String,
    filename: String,
}

impl Config {
    // the new function is a convention for a constructor in Rust
    fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}
