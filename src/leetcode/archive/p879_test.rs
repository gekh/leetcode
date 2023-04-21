use super::p879::Solution;

pub fn test() {
    assert_eq!(call(5,3,vec![2,2], vec![2,3]), 2);
    assert_eq!(call(10,5,vec![2,3,5], vec![6,7,8]), 7);
    assert_eq!(call(10,5,vec![2,3,5], vec![1,2,3]), 2);
    assert_eq!(call(5,5,vec![2,3,5], vec![6,7,8]), 4);
    assert_eq!(call(64,0,vec![80,40], vec![88,88]), 2);
}

fn call(n: i32, min_profit: i32, group: Vec<i32>, profit: Vec<i32>) -> i32 {
  Solution::profitable_schemes(n, min_profit, group, profit)
}
