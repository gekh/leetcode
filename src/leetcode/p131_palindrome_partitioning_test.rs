use super::p131_palindrome_partitioning::Solution;

pub fn test() {
    assert_eq!(
        call("aab"),
        vec![
            vec![String::from("a"), String::from("a"), String::from("b")],
            vec![String::from("aa"), String::from("b")],
        ]
    );

    assert_eq!(call("a"), vec![vec![String::from("a")],]);

    assert_eq!(call("a"), vec![vec![String::from("a")],]);

    assert_eq!(
        call("bacab"),
        vec![
            vec![String::from("b"), String::from("a"), String::from("c"), String::from("a"), String::from("b")],
            vec![String::from("b"), String::from("aca"), String::from("b")],
            vec![String::from("bacab")]
        ]
    );

    assert_eq!(
        call("aaaa"),
        vec![
            vec![String::from("a"),String::from("a"),String::from("a"),String::from("a")],
            vec![String::from("a"),String::from("a"),String::from("aa")],
            vec![String::from("a"),String::from("aa"),String::from("a")],
            vec![String::from("a"),String::from("aaa")],
            vec![String::from("aa"),String::from("a"),String::from("a")],
            vec![String::from("aa"),String::from("aa")],
            vec![String::from("aaa"),String::from("a")],
            vec![String::from("aaaa")]
        ]
    );
}

fn call(s: &str) -> Vec<Vec<String>> {
    Solution::partition(String::from(s))
}
