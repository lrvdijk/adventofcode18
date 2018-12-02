extern crate adventofcode18;

use std::env;
use std::process;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::error::Error;

use adventofcode18::skip_characters;

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


fn determine_freq(filename: &str) -> Result<i32, Box<dyn Error>> {
    let mut frequency: i32 = 0;
    let f = File::open(filename)?;

    let reader = BufReader::new(&f);

    for line in reader.lines() {
        let l = line?;

        let adjust: i32 = if l.starts_with("+") {
            skip_characters(&l, 1).trim().parse()?
        } else {
            -skip_characters(&l, 1).trim().parse()?
        };

        frequency += adjust;
    }

    Ok(frequency)
}
