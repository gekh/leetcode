pub struct Solution;

impl Solution {
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        if nums.len() == 1 {
            return vec![nums];
        } else if nums.len() == 2 {
            return vec![vec![nums[0], nums[1]], vec![nums[1], nums[0]]];
        }

        let mut result = vec![];
        for &el in &nums {
            let new_nums = nums
                .iter()
                .filter(|&&inel| inel != el)
                .map(|inel| inel.clone())
                .collect::<Vec<_>>();
            for mut rel in Self::permute(new_nums) {
                rel.push(el);
                result.push(rel);
            }
        }

        result
    }
}
