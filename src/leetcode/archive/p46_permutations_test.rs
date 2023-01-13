use super::p46_permutations::Solution;

pub fn test() {
    let mut result = call(vec![1,2,3,4]);
    result.sort();
    assert_eq!(result, vec![vec![1, 2, 3, 4], vec![1, 2, 4, 3], vec![1, 3, 2, 4], vec![1, 3, 4, 2], vec![1, 4, 2, 3], vec![1, 4, 3, 2], vec![2, 1, 3, 4], vec![2, 1, 4, 3], vec![2, 3, 1, 4], vec![2, 3, 4, 1], vec![2, 4, 1, 3], vec![2, 4, 3, 1], vec![3, 1, 2, 4], vec![3, 1, 4, 2], vec![3, 2, 1, 4], vec![3, 2, 4, 1], vec![3, 4, 1, 2], vec![3, 4, 2, 1], vec![4, 1, 2, 3], vec![4, 1, 3, 2], vec![4, 2, 1, 3], vec![4, 2, 3, 1], vec![4, 3, 1, 2], vec![4, 3, 2, 1]]);

    // let mut result = call(vec![1,2,3]);
    // result.sort();
    // assert_eq!(result, vec![vec![1,2,3],vec![1,3,2],vec![2,1,3],vec![2,3,1],vec![3,1,2],vec![3,2,1]]);

    // let mut result = call(vec![0,1]);
    // result.sort();
    // assert_eq!(result, vec![vec![0,1],vec![1,0]]);

    // let mut result = call(vec![1]);
    // result.sort();
    // assert_eq!(result, vec![vec![1]]);
}

fn call(nums: Vec<i32>) -> Vec<Vec<i32>> {
    Solution::permute(nums)
}
