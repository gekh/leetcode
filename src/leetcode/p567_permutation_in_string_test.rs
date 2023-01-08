use super::p567_permutation_in_string::{Solution};

pub fn test() {
    assert_eq!(call("lol", "hellothere"), true);
    assert_eq!(call("ab", "eidbaooo"), true);
    assert_eq!(call("ab", "eidboaoo"), false);
    assert_eq!(call("abd", "eibadoaoo"), true);
    assert_eq!(call("a", "eibadoaoo"), true);
    assert_eq!(call("horse", "ros"), false);
}

fn call(s1: &str, s2: &str) -> bool {
    Solution::check_inclusion(String::from(s1), String::from(s2))
}
