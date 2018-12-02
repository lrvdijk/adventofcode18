use std::io::{Read, BufRead, BufReader};
use std::num::ParseIntError;
use std::iter::Iterator;

use super::skip_characters;


pub fn frequency_adjustments_iter<T>(input: T) -> impl Iterator<Item=isize>
    where T: Read + Sized
{
    let reader = BufReader::new(input);
    return reader.lines()
        .filter_map(|l| {
            match l {
                Ok(line) => {
                    parse_frequency(&line.trim()).ok()
                }
                Err(_) => None
            }
        })
}


pub fn parse_frequency(line: &str) -> Result<isize, ParseIntError> {
    if line.starts_with('+') {
        skip_characters(line, 1).parse()
    } else {
        line.trim().parse()
    }
}



