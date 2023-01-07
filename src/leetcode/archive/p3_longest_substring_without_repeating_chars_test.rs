use super::p3_longest_substring_without_repeating_chars::{Solution};

pub fn test() {
    assert_eq!(call("abcabcbb"), 3);
    assert_eq!(call("bbbbb"), 1);
    assert_eq!(call("pwwkew"), 3);
    assert_eq!(call(" "), 1);
    assert_eq!(call("au"), 2);
    assert_eq!(call("dvdf"), 3);
}

fn call(s: &str) -> i32 {
    Solution::length_of_longest_substring(String::from(s))
}
