pub struct Solution;

impl Solution {
    pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
        let mut result = vec![];
        let mut window = vec![];
        Self::recursion(&mut result, &mut window, k as usize, 1, n - k + 1);

        result
    }

    fn recursion(
        mut result: &mut Vec<Vec<i32>>,
        mut window: &mut Vec<i32>,
        k: usize,
        start: i32,
        end: i32,
    ) {
        if window.len() == k {
            result.push(window.clone());
            return;
        }
        for i in start..=end {
            window.push(i);
            Self::recursion(&mut result, &mut window, k, i + 1, end + 1);
            window.pop();
        }
    }
}
