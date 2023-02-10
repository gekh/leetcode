use super::p1162_as_far_from_land_as_possible::Solution;

pub fn test() {
    assert_eq!(call(vec![vec![1,0,1],vec![0,0,0],vec![1,0,1]]), 2);
    assert_eq!(call(vec![vec![1,0,0],vec![0,0,0],vec![0,0,0]]), 4);
    assert_eq!(call(vec![vec![0,0,0],vec![0,0,0],vec![0,0,1]]), 4);
    assert_eq!(call(vec![vec![0,0,0],vec![0,0,0],vec![0,0,0]]), -1);
    assert_eq!(call(vec![vec![1,1,1],vec![1,1,1],vec![1,1,1]]), -1);
    assert_eq!(call(vec![vec![0,1,0,0],vec![0,0,1,0],vec![0,0,0,1],vec![0,1,0,0]]), 2);

}

fn call(grid: Vec<Vec<i32>>) -> i32 {
    Solution::max_distance(grid)
}
