pub struct Solution();

impl Solution {
    pub fn word_pattern(pattern: String, s: String) -> bool {
        if pattern.len() != s.matches(' ').count() + 1 {
            return false;
        }

        let chars = pattern.chars();
        let words = s.split_whitespace();

        let mut keys: Vec<char> = vec![];
        let mut values: Vec<&str> = vec![];

        for (c,w) in chars.into_iter().zip(words.into_iter()) {

            match (
                keys.iter().position(|&x| x == c),
                values.iter().position(|&x| x == w),
            ) {
                (None, None) => {
                    keys.push(c);
                    values.push(w);
                }
                (Some(_), None) | (None, Some(_)) => {
                    return false;
                }
                (Some(k), Some(v)) => {
                    if k != v {
                        return false;
                    }
                }
            }
        }

        true
    }
}
