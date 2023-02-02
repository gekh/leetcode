use super::p909_snakes_and_ladders::Solution;

pub fn test() {
    assert_eq!(call(vec![vec![-1,1,1,1],vec![-1,7,1,1],vec![16,1,1,1],vec![-1,1,9,1]]), 3);
    assert_eq!(call(vec![vec![-1,-1,-1],vec![-1,-1,-1],vec![-1,-1,-1]]), 2);
    assert_eq!(call(vec![vec![1,-1, -1],vec![-1,-1,-1],vec![-1,-1,-1]]), 2);
    assert_eq!(call(vec![vec![-1,-1,-1,-1,-1,-1],vec![-1,-1,-1,-1,-1,-1],vec![-1,-1,-1,-1,-1,-1],vec![-1,35,-1,-1,13,-1],vec![-1,-1,-1,-1,-1,-1],vec![-1,15,-1,-1,-1,-1]]), 4);
    assert_eq!(call(vec![vec![-1,-1,-1,-1,-1,-1],vec![-1,-1,-1,-1,-1,-1],vec![-1,-1,-1,-1,-1,-1],vec![-1,35,-1,-1,13,-1],vec![-1,-1,-1,-1,36,-1],vec![-1,15,-1,-1,-1,-1]]), 2);
    assert_eq!(call(vec![vec![-1,-1,-1,-1,-1],vec![-1,-1,-1,-1,-1],vec![-1,24,-1,-1,13],vec![-1,-1,-1,-1,-1],vec![-1,15,-1,-1,-1]]), 3);
    assert_eq!(call(vec![vec![-1,1],vec![-1,3]]), 1);
    assert_eq!(call(vec![vec![-1,-1,-1,-1,-1,-1],vec![-1,-1,-1,-1,-1,-1],vec![-1,-1,-1,-1,-1,-1],vec![-1,-1,35,-1,14,-1],vec![-1,-1,-1,-1,-1,-1],vec![-1,16,-1,-1,-1,-1]]), 4);
    assert_eq!(call(vec![vec![1,1,-1],vec![1,1,1],vec![-1,1,1]]), -1);
    assert_eq!(call(vec![vec![-1,10,-1,15,-1],vec![-1,-1,18,2,20],vec![-1,-1,12,-1,-1],vec![2,4,11,18,8],vec![-1,-1,-1,-1,-1]]), 3);
}

fn call(board: Vec<Vec<i32>>) -> i32 {
    Solution::snakes_and_ladders(board)
}