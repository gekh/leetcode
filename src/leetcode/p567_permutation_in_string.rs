

pub struct Solution;

impl Solution {
    pub fn check_inclusion(s1: String, s2: String) -> bool {
        let (mut a1, mut a2) = ([0;26], [0;26]);

        for c in s1.as_bytes() {
            a1[(c - b'a') as usize] += 1;
        }

        let s2b = s2.as_bytes();
        for (i,c) in s2b.iter().enumerate() {
            a2[(c - b'a') as usize] += 1;

            if i > s1.len() - 1 {
                a2[(s2b[i - s1.len()] - b'a') as usize] -= 1;
            }

            if a1 == a2 {
                return true;
            }
        }

        false
    }
}