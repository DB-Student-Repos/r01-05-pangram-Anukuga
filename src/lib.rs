/// Determine whether a sentence is a pangram.
pub fn is_pangram(sentence: &str) -> bool {
    let alphabet = "abcdefghijklmnopqrstuvwxyz";
     alphabet.chars().all(|c| sentence.to_lowercase().contains(c))
}
