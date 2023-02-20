use super::p::Solution;

pub fn test() {
    // assert_eq!(call(), 0);
}

fn call(words: Vec<String>) -> Vec<String> {
  Solution::find_all_concatenated_words_in_a_dict(words)
}
