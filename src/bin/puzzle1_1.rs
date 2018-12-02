extern crate adventofcode18;

use std::env;
use std::process;
use std::fs::File;
use std::error::Error;

use adventofcode18::frequency;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} [filename]", args[0]);
        process::exit(1);
    }

    let frequency = determine_freq(&args[1]).unwrap_or_else(|err| {
        eprintln!("Error reading file: {}", err);
        process::exit(1);
    });

    println!("Frequency: {}", frequency);
}


fn determine_freq(filename: &str) -> Result<isize, Box<dyn Error>> {
    let f = File::open(filename)?;
    let frequency: isize = frequency::frequency_adjustments_iter(&f).sum();

    Ok(frequency)
}
