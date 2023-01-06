pub struct Solution;

impl Solution {
    pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
        let mut answer = vec![];
        let mut left: isize = 0;
        let mut right = nums.len() as isize - 1;

        while left <= right {
            let labs = nums[left as usize].abs();
            let rabs = nums[right as usize].abs();
            if labs > rabs {
                answer.push(labs.pow(2));
                left += 1;
            } else {
                answer.push(rabs.pow(2));
                right -= 1;
            }
        }

        answer.reverse();
        return answer;
    }
}
