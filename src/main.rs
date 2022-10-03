use tsp::config::Config;
use std::{env, process};

/// usage: cargo run <filename>
/// 
/// Entry point to the program. Accepts one command line argument representing
/// the name of the file containing the coordinates of cities to be traversed.
/// Returns an error if the wrong number of command line arguments is provided
/// or if the file cannot be parsed according to the required format.
fn main() -> Result<(), &'static str> {
    let config = Config::from_args(env::args());
    if let Err(_) = config {
        eprintln!("usage: cargo run <file> <pop_size> <num_evals>");
        process::exit(1);
    }

    let result = tsp::run(config.unwrap());

    if let Err(err) = result {
        eprintln!("error while processing: {}", err);
        process::exit(1);
    };
    Ok(())
}
