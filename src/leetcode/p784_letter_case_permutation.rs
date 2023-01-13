pub struct Solution;

impl Solution {
    pub fn letter_case_permutation(s: String) -> Vec<String> {
        let mut result = vec![];
        Self::backtrack(&s, &mut vec![], &mut result, 0);

        result
    }

    fn backtrack(s: &String, mut list: &mut Vec<char>, mut result: &mut Vec<String>, start: usize) {
        if list.len() == s.len() {
            result.push(list.iter().collect::<String>());
            return;
        }
        for (i, c) in s.chars().skip(start).enumerate() {
            let i = i + start;
            if c.is_alphabetic() {
                let up = c.to_ascii_uppercase();
                list.push(up);
                Self::backtrack(&s, &mut list, &mut result, i + 1);
                list.pop();

                let down = c.to_ascii_lowercase();
                list.push(down);
                Self::backtrack(&s, &mut list, &mut result, i + 1);
                list.pop();
                break;
            } else {
                list.push(c);
                Self::backtrack(&s, &mut list, &mut result, i + 1);
                list.pop();
            }
        }
    }
}
