pub struct Solution;

impl Solution {
    pub fn find_subsequences(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result: Vec<Vec<i32>> = vec![];

        for &num in nums.iter().rev() {
            for i in 0..result.len() {
                if num <= result[i][0] {
                    let mut sub = result[i].clone();
                    sub.insert(0, num);
                    result.push(sub);
                }
            }
            result.push(vec![num]);
        }

        result = result
            .into_iter()
            .filter(|el| el.len() > 1)
            .collect::<Vec<_>>();
        result.sort_unstable();
        result.dedup();

        result
    }
}
