use std::{env, process};

/// usage: cargo run <filename>
/// 
/// Entry point to the program. Accepts one command line argument representing
/// the name of the file containing the coordinates of cities to be traversed.
/// Returns an error if the wrong number of command line arguments is provided
/// or if the file cannot be parsed according to the required format.
fn main() -> Result<(), &'static str> {
    let mut args = env::args();
    let _ = args.next();
    let file = args.next();
    if let None = file {
        eprintln!("usage: cargo run <file>");
        process::exit(1);
    }

    let result = tsp::run(&file.unwrap());

    if let Err(err) = result {
        eprintln!("error while processing: {}", err);
        process::exit(1);
    };
    Ok(())
}
