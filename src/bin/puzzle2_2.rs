extern crate adventofcode18;

use std::env;
use std::process;
use std::fs::File;
use std::error::Error;
use std::io::{BufReader, BufRead};

use adventofcode18::closest_string::closest_two_strings;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} [input filename]", args[0]);
        process::exit(1);
    }

    let mut box_ids = get_box_ids(&args[1]).unwrap_or_else(|err| {
        eprintln!("Could not read box ids: {}", err);
        process::exit(1);
    });
    box_ids.sort();

    if let Some((string1, string2)) = closest_two_strings(&box_ids) {
        let in_common = string1.chars().zip(string2.chars())
            .filter_map(|(a, b)| {
                if a == b { Some(a) } else { None }
            })
            .collect::<String>();

        println!("In common: {}", in_common);
    } else {
        eprintln!("No two strings found with hamming distance 1");
        process::exit(1);
    }
}


fn get_box_ids(filename: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    Ok(reader.lines()
        .filter_map(|l| {
            match l {
                Ok(line) => {
                    Some(line.trim().to_string())
                }
                Err(_) => None
            }
        }).collect()
    )
}