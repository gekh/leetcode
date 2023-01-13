use super::p120_triangle::Solution;

pub fn test() {
    assert_eq!(call(vec![vec![2],vec![3,4],vec![6,5,7],vec![4,1,8,3]]), 11);
    assert_eq!(call(vec![vec![-10]]), -10);
    assert_eq!(call(vec![vec![-7],vec![-2,1],vec![-5,-5,9],vec![-4,-5,4,4],vec![-6,-6,2,-1,-5],vec![3,7,8,-3,7,-9],vec![-9,-1,-9,6,9,0,7],vec![-7,0,-6,-8,7,1,-4,9],vec![-3,2,-6,-9,-7,-6,-9,4,0],vec![-8,-6,-3,-9,-2,-6,7,-5,0,7],vec![-9,-1,-2,4,-2,4,4,-1,2,-5,5],vec![1,1,-6,1,-2,-4,4,-2,6,-6,0,6],vec![-3,-3,-6,-2,-6,-2,7,-9,-5,-7,-5,5,1]]), -63);
}

fn call(triangle: Vec<Vec<i32>>) -> i32 {
    Solution::minimum_total(triangle)
}
