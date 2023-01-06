pub struct Solution;

impl Solution {
    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        if k == 0 {
            return;
        }

        let length = nums.len() as i32;
        let k = (k % length) as i32;

        Self::reverse(nums, 0, length-1);
        Self::reverse(nums, 0, k-1);
        Self::reverse(nums, k, length-1);
    }

    fn reverse(nums: &mut Vec<i32>, mut l: i32, mut r: i32) {
        while l < r {
            nums.swap(l as usize,r as usize);
            l += 1;
            r -= 1;
        }
    }
}