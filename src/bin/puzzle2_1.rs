extern crate adventofcode18;

use std::env;
use std::process;
use std::fs::File;
use std::error::Error;
use std::io::{BufReader, BufRead};

use adventofcode18::{CharDist, character_counter};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} [input filename]", args[0]);
        process::exit(1);
    }

    let checksum = calc_checksum(&args[1]).unwrap_or_else(|err| {
        eprintln!("Error calculating checksum: {}", err);
        process::exit(1);
    });

    println!("Checksum: {}", checksum);
}


fn calc_checksum(filename: &str) -> Result<isize, Box<dyn Error>> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    let char_counters: Vec<CharDist> = reader.lines()
        .filter_map(|l| {
            match l {
                Ok(line) => {
                    Some(character_counter(line.trim()))
                }
                Err(_) => None
            }
        }).collect();

    let mut num2 = 0;
    let mut num3 = 0;
    for char_dist in &char_counters {
        let mut has2 = false;
        let mut has3 = false;

        for (_, value) in char_dist.into_iter() {
            if *value == 2 {
                has2 = true;
            }

            if *value == 3 {
                has3 = true;
            }
        }

        if has2 {
            num2 += 1;
        }

        if has3 {
            num3 += 1;
        }
    }

    return Ok(num2 * num3);
}