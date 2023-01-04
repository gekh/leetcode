pub struct Solution;

impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let mut ball = 0;
        for i in 0..nums.len() {
            if nums[i] != 0 {
                if i != ball {
                    nums.swap(i,ball);
                }
                ball += 1;
            }
        }
    }
}
