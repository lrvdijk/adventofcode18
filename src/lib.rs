pub mod frequency;

/// Skip the first `num` characters of a given string slice
pub fn skip_characters(string: &str, num: usize) -> &str {
    match string.char_indices().skip(num).next() {
        Some((pos, _)) => { &string[pos..] }
        None => ""
    }
}