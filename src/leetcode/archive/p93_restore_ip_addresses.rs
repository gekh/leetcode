pub struct Solution;

impl Solution {
    pub fn restore_ip_addresses(s: String) -> Vec<String> {
        if s.len() < 4 || s.len() > 12 {
            return vec![];
        }

        let mut stack = vec![];
        let mut result = vec![];
        Self::backtrack(&s, 0, &mut stack, &mut result);

        result
    }

    fn backtrack(
        s: &String,
        start: usize,
        mut stack: &mut Vec<String>,
        mut result: &mut Vec<String>,
    ) {
        if stack.len() == 4 {
            if start == s.len() {
                result.push(stack.join("."));
            }
            return;
        }

        Self::sub(&s, start, start + 1, &mut stack, &mut result);
        Self::sub(&s, start, start + 2, &mut stack, &mut result);
        Self::sub(&s, start, start + 3, &mut stack, &mut result);
    }

    fn sub(
        s: &String,
        start: usize,
        end: usize,
        mut stack: &mut Vec<String>,
        mut result: &mut Vec<String>,
    ) {
        if end > s.len() { return }
        let el = s[start..end].to_string();
        if el.len() == 3 && el.parse::<u16>().unwrap() > 255 { return }
        if el.len() > 1 && el.as_bytes()[0] == 48 { return }

        stack.push(el);
        Self::backtrack(&s, end, &mut stack, &mut result);
        stack.pop();
    }
}
