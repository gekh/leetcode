pub struct Solution;

impl Solution {
    pub fn is_alien_sorted(words: Vec<String>, order: String) -> bool {
        if words.len() <= 1 { return true; }

        let mut m = vec![0;26];
        for (i, &c) in order.as_bytes().iter().enumerate() {
            m[(c - b'a') as usize] = i;
        }

        let words: Vec<Vec<usize>> = words
            .iter()
            .map(|w|
                w.as_bytes()
                .iter()
                .map(|c| m[(c - b'a') as usize])
                .collect::<Vec<usize>>()
            )
            .collect();


        for i in 1..words.len() {
            if words[i-1] > words[i] {
                return false;
            }
        }

        true
    }
}
