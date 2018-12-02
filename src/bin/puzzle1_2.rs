extern crate adventofcode18;

use std::env;
use std::process;
use std::fs::File;
use std::error::Error;
use std::collections::HashSet;

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
    let adjustments: Vec<isize> = frequency::frequency_adjustments_iter(&f).collect();

    let mut frequency = 0;
    let mut observed = HashSet::new();
    let mut i = 0;

    loop {
        if observed.contains(&frequency) {
            return Ok(frequency);
        }

        observed.insert(frequency.clone());
        frequency += adjustments[i];

        i = if i == adjustments.len() - 1 {
            0
        } else {
            i + 1
        };
    }
}
