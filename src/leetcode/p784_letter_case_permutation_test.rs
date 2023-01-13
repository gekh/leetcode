use super::p784_letter_case_permutation::Solution;

pub fn test() {
    let mut result = call("a1b2");
    result.sort();
    assert_eq!(result, vec!["A1B2", "A1b2", "a1B2", "a1b2"]);

    let mut result = call("3z4");
    result.sort();
    assert_eq!(result, vec!["3Z4", "3z4"]);

    let mut result = call("z");
    result.sort();
    assert_eq!(result, vec!["Z", "z"]);

    let mut result = call("34");
    result.sort();
    assert_eq!(result, vec!["34"]);
}

fn call(s: &str) -> Vec<String> {
    Solution::letter_case_permutation(String::from(s))
}
