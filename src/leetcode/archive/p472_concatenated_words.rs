pub struct Solution;

use std::collections::HashSet;
use std::iter::FromIterator;
impl Solution {
    pub fn find_all_concatenated_words_in_a_dict(words: Vec<String>) -> Vec<String> {
        let mut result = vec![];
        let hs: HashSet<&str> = HashSet::from_iter(words.iter().map(|s| s.as_str()));
        for w in &words {
            if let Some(depth) = Self::dfs(&hs, w) {
                if depth != 1 {
                    result.push(w.clone());
                }
            }
        }
        result
    }

    fn dfs(hs: &HashSet<&str>, word: &str) -> Option<u32> {
        for i in 1..=word.len() {
            if hs.contains(&word[..i]) {
                if i == word.len() {
                    return Some(1);
                } else if let Some(depth) = Self::dfs(hs, &word[i..]) {
                    return Some(depth + 1);
                }
            }
        }

        None
    }
}
