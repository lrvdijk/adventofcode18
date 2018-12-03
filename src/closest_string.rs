/// Finds two strings that are different by at most 1 character
/// at the same position. In other words, the hamming distance
/// should be one for those two.
///
/// `strings` should be sorted.
pub fn closest_two_strings(strings: &[String]) -> Option<(&str, &str)> {
    for i in 0..strings.len()-1 {
        if let Some(dist) = hamming_distance(&strings[i], &strings[i+1]) {
            if dist == 1 {
                return Some((strings[i].as_str(), strings[i+1].as_str()));
            }
        }
    }

    None
}


pub fn hamming_distance(string1: &str, string2: &str) -> Option<isize> {
    if string1.len() != string2.len() {
        return None;
    }

    Some(
        string1.chars().zip(string2.chars())
            .map(|(a, b)| {
                if a == b { 0 } else { 1 }
            })
            .sum()
    )
}
