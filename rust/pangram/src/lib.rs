use itertools::Itertools;
const NUM_ALPHA: usize = 26;

/// Determine whether a sentence is a pangram.
pub fn is_pangram(sentence: &str) -> bool {
    sentence
        .chars()
        .filter(|c| c.is_ascii() && c.is_alphabetic())
        .map(|c| c.to_ascii_lowercase())
        .unique()
        .count() == NUM_ALPHA
}
