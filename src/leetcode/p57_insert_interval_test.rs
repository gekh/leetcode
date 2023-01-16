use super::p57_insert_interval::Solution;

pub fn test() {
    assert_eq!( call(vec![vec![1, 3], vec![6, 9]], vec![2, 5]), vec![vec![1, 5], vec![6, 9]] );
}

fn call(intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
    Solution::insert(intervals, new_interval)
}
