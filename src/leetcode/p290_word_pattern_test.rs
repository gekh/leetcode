use super::p290_word_pattern::Solution;

pub fn test() {
    assert!(call("abba", "dog cat cat dog") == true);
    assert!(call("abba", "dog cat cat cat") == false);
    assert!(call("aaaa", "cat cat cat cat") == true);
    assert!(call("aaaa", "cat cat cat") == false);
    assert!(call("aaa", "cat cat cat cat") == false);
    assert!(call("abba", "dog dog dog dog") == false);
    assert!(call("aaaa", "dog cat cat dog") == false);
}

fn call(pattern: &str, s: &str) -> bool {
    Solution::word_pattern(String::from(pattern), String::from(s))
}
