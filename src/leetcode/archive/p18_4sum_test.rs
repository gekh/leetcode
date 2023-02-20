use super::p18_4sum::Solution;

pub fn test() {
  assert_eq!( call(vec![1, 0, -1, 0, -2, 2], 0), vec![vec![-2, -1, 1, 2], vec![-2, 0, 0, 2], vec![-1, 0, 0, 1]] );
  assert_eq!( call(vec![2,2,2,2,2], 8), vec![vec![2,2,2,2]] );
}

fn call(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
  Solution::four_sum(nums, target)
}
