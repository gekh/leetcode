use super::p1626_best_team_with_no_conflicts::Solution;

pub fn test() {
    assert_eq!(call(vec![1,3,5,10,15], vec![1,2,3,4,5]), 34);
    assert_eq!(call(vec![1,3,5,10,15], vec![1,2,5,4,3]), 19);
    assert_eq!(call(vec![4,5,6,5], vec![2,1,2,1]), 16);
    assert_eq!(call(vec![1,2,3,5], vec![8,9,10,1]), 6);
}

fn call(scores: Vec<i32>, ages: Vec<i32>) -> i32 {
    Solution::best_team_score(scores, ages)
}
