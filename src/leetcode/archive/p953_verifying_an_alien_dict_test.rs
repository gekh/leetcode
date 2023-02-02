use super::p953_verifying_an_alien_dict::Solution;

pub fn test() {
    assert_eq!(call(vec![String::from("hlabcde"),String::from("hla"),String::from("hlabcdh")], String::from("hlabcdefgijkmnopqrstuvwxyz")), false);
    assert_eq!(call(vec![String::from("hello"),String::from("leetcode")], String::from("hlabcdefgijkmnopqrstuvwxyz")), true);
    assert_eq!(call(vec![String::from("word"),String::from("world"),String::from("row")], String::from("worldabcefghijkmnpqstuvxyz")), false);
    assert_eq!(call(vec![String::from("apple"),String::from("app")], String::from("abcdefghijklmnopqrstuvwxyz")), false);
}

fn call(words: Vec<String>, order: String) -> bool {
    Solution::is_alien_sorted(words, order)
}
