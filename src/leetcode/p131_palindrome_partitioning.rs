pub struct Solution;

impl Solution {
    pub fn partition(s: String) -> Vec<Vec<String>> {
        let (mut stack, mut result) = (vec![], vec![]);
        Self::backtrack(0, &s, &mut result, &mut stack);

        result
    }

    fn backtrack(start: usize, s: &String, mut result: &mut Vec<Vec<String>>, mut stack: &mut Vec<String>) {
        if start >= s.len() {
            result.push(stack.clone());
            return;
        }

        for i in (start + 1)..=s.len() {
            if Self::is_palindrome(&s[start..i]) {
                stack.push(s[start..i].to_string());
                Self::backtrack(i, &s, &mut result, &mut stack);
                stack.pop();
            }
        }
    }

    fn is_palindrome(s: &str) -> bool {
        let (mut l, mut r) = (0, s.len() - 1);
        while l < r {
            if &s[l..l+1] != &s[r..r+1] {
                return false;
            }
            l += 1;
            r -= 1;
        }

        true
    }
}
