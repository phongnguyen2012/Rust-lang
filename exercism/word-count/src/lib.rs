use std::collections::HashMap;
/// Count occurrences of words.
pub fn word_count(words: &str) -> HashMap<String, u32> {
    let mut counts = HashMap::new();
    let cleaned_words: String = words.chars()
        .map(|char| filter_punctuation(char))
        .collect();
    cleaned_words.split_whitespace()
        .map(|word| word.trim_matches('\''))
        .for_each(|word|{
            let count = counts.entry(word.to_lowercase()).or_insert(0);
            *count+=1;
    });
    counts
}
pub fn filter_punctuation(char: char) -> char {
    return if char.is_ascii_punctuation() && char!='\'' {
        ' '
    } else {
        char
    }
}
