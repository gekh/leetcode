pub struct Solution;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut v = Vec::new();
        let mut max = 0;
        for (_i, c) in s.chars().enumerate() {
            match v.iter().position(|&el| el == c) {
                Some(pos) => {
                    v.drain(..(pos + 1));
                }
                None => (),
            };

            v.push(c);

            if max < v.len() {
                max = v.len();
            }
        }

        max as _
    }
}
