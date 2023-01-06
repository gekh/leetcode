use super::p167_two_sum_ii::Solution;

pub fn test() {
    assert!(call(vec![2,7,11,15], 9) == vec![1,2]);
}

fn call(numbers: Vec<i32>, target: i32) -> Vec<i32> {
    Solution::two_sum(numbers, target)
}
