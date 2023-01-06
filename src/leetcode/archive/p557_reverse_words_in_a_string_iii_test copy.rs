use super::p557_reverse_words_in_a_string_iii::Solution;

pub fn test() {
    assert_eq!(call("Let's take LeetCode contest"), String::from("s'teL ekat edoCteeL tsetnoc"))
}

fn call(s: &str) -> String {
    Solution::reverse_words(String::from(s))
}
