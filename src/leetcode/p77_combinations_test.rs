use super::p77_combinations::Solution;

pub fn test() {
    // assert_eq!(call(7, 3), vec![vec![1, 2], vec![1, 3], vec![1, 4], vec![2, 3], vec![2, 4], vec![3, 4]]);
    assert_eq!(call(4, 2), vec![vec![1, 2], vec![1, 3], vec![1, 4], vec![2, 3], vec![2, 4], vec![3, 4]]);
    assert_eq!(call(1, 1), vec![vec![1]]);
    assert_eq!(call(5, 1), vec![vec![1],vec![2],vec![3],vec![4],vec![5]]);
}

fn call(n: i32, k: i32) -> Vec<Vec<i32>> {
    Solution::combine(n, k)
}
