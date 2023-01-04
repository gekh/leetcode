pub struct Solution;

impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let mut left = 0;
        let mut right = numbers.len() - 1;
        while left < right {
            let sum = numbers[left] + numbers[right];
            if sum == target {
                return [left+1, right+1].iter().map(|x| *x as i32).collect::<Vec<i32>>();
            } else if sum > target {
                right -= 1;
            } else {
                left += 1;
            }
        }
        vec![]
    }
}