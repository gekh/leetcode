use super::p15_3sum::Solution;

pub fn test() {
  assert_eq!(call( vec![-1,0,1,2,-1,-4] ), vec![vec![-1,-1,2],vec![-1,0,1]] );
  let empty: Vec<Vec<i32>> = vec![];
  assert_eq!(call( vec![0,1,1] ), empty);
  assert_eq!(call( vec![0,0,0] ), vec![vec![0,0,0]] );
  assert_eq!(call( vec![-1,-1,-1,-1,2,2,2,2] ), vec![vec![-1,-1,2]] );
  assert_eq!(call( vec![1,0,-1,-2,1,4] ), vec![vec![-2,1,1],vec![-1,0,1]] );
  assert_eq!(call( vec![3,0,-2,-1,1,2] ), vec![vec![-2,-1,3],vec![-2,0,2],vec![-1,0,1]] );
  assert_eq!(call( vec![-4,-2,-2,-2,0,1,2,2,2,3,3,4,4,6,6] ), vec![vec![-4,-2,6],vec![-4,0,4],vec![-4,1,3],vec![-4,2,2],vec![-2,-2,4],vec![-2,0,2]] );
}

fn call(nums: Vec<i32>) -> Vec<Vec<i32>> {
    Solution::three_sum(nums)
}
