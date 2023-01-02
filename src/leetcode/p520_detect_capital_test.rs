use super::p520_detect_capital::Solution;

pub fn test() {
    assert!(call("USA") == true);
    assert!(call("cabbage") == true);
    assert!(call("India") == true);
    assert!(call("MaroO") == false);
    assert!(call("mousE") == false);
    assert!(call("gnOMe") == false);
    assert!(call("cAt") == false);
    assert!(call("mL") == false);
}

fn call(s: &str) -> bool {
    Solution::detect_capital_use(String::from(s))
}
