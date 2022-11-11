use std::collections::HashSet;
pub fn anagrams_for<'a>(word: &str, possible_anagrams: &'a[&str]) -> HashSet<&'a str> {
    let lowercase_word = word.clone().to_lowercase();
    let mut word_chars = lowercase_word.chars().collect::<Vec<_>>();
    word_chars.sort_unstable();
    let mut anagrams = HashSet::new();
    
    for &possible_anagram in possible_anagrams {
        let lowercase_anagram = possible_anagram.to_lowercase();
        if lowercase_word != lowercase_anagram { 
            let mut anagram_chars = lowercase_anagram.chars().collect::<Vec<_>>();
            anagram_chars.sort_unstable();
            if word_chars == anagram_chars { 
                anagrams.insert(possible_anagram);
            }
        }
    }
   return anagrams;
}
