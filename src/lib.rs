use std::collections::HashMap;

pub mod frequency;
pub mod closest_string;

/// Skip the first `num` characters of a given string slice
pub fn skip_characters(string: &str, num: usize) -> &str {
    match string.char_indices().skip(num).next() {
        Some((pos, _)) => { &string[pos..] }
        None => ""
    }
}


pub type CharDist = HashMap<char, isize>;

pub fn character_counter(string: &str) -> CharDist {
    let mut counter = CharDist::new();

    for char in string.chars() {
        *counter.entry(char).or_insert(0) += 1;
    }

    counter
}
