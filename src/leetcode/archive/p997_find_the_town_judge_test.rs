use super::p997_find_the_town_judge::Solution;

pub fn test() {
    assert_eq!(call(2, vec![vec![1,2]]), 2);
    assert_eq!(call(3, vec![vec![1,3],vec![2,3]]), 3);
    assert_eq!(call(3, vec![vec![1,3],vec![2,3],vec![3,1]]), -1);
}

fn call(n: i32, trust: Vec<Vec<i32>>) -> i32 {
    Solution::find_judge(n, trust)
}
