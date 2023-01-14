use super::p1061_lexicographically_smallest_equivalent_string::Solution;

pub fn test() {

    assert_eq!(call("abca", "defc", "fed"), String::from("aba"));
    assert_eq!(call("parker", "morris", "parser"), String::from("makkek"));
    assert_eq!(call("hello", "world", "hold"), String::from("hdld"));
    assert_eq!(call("leetcode", "programs", "sourcecode"), String::from("aauaaaaada"));
    assert_eq!(call(
        "gmerjboftfnqseogigpdnlocmmhskigdtednfnjtlcrdpcjkbj",
        "fnnafafhqkitbcdlkpiloiienikjiikdfcafisejgeldprcmhd",
        "ezrqfyguivmybqcsvibuvtajdvamcdjpmgcbvieegpyzdcypcx"
    ), String::from(
        "azaaayauavayaaaavaauvaaaavaaaaaaaaaavaaaaayzaayaax"
    ));
}

fn call<'a>(s1: &str, s2: &str, base_str: &str) -> String {
    Solution::smallest_equivalent_string(String::from(s1), String::from(s2), String::from(base_str))
}
