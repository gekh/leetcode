use super::p472_concatenated_words::Solution;

pub fn test() {
    assert_eq!(
        call(vec![
            String::from("cat"),
            String::from("cats"),
            String::from("catsdogcats"),
            String::from("dog"),
            String::from("dogcatsdog"),
            String::from("dogcatsdogcats"),
            String::from("hippopotamuses"),
            String::from("rat"),
            String::from("ratcatdogcat"),
            String::from("ratcatdogcats"),
        ]),
        vec![
            String::from("catsdogcats"),
            String::from("dogcatsdog"),
            String::from("dogcatsdogcats"),
            String::from("ratcatdogcat"),
            String::from("ratcatdogcats"),
        ]
    );
    assert_eq!(
        call(vec![
            String::from("cat"),
            String::from("dog"),
            String::from("catdog")
        ]),
        vec![String::from("catdog")]
    );
    assert_eq!(
        call(vec![
            String::from("a"),
            String::from("aa"),
            String::from("b"),
            String::from("c"),
            String::from("ab"),
            String::from("abc"),
            String::from("bbb"),
            String::from("xyz"),
            String::from("xyza"),
            String::from("xbyza"),
        ]),
        vec![
            String::from("aa"),
            String::from("ab"),
            String::from("abc"),
            String::from("bbb"),
            String::from("xyza"),
        ]
    );
}

fn call(words: Vec<String>) -> Vec<String> {
    Solution::find_all_concatenated_words_in_a_dict(words)
}
