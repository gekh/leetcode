use super::p491_non_desc_subsequences::Solution;

pub fn test() {
    let mut result = vec![vec![4,6],vec![4,6,7],vec![4,6,7,7],vec![4,7],vec![4,7,7],vec![6,7],vec![6,7,7],vec![7,7]];
    result.sort_unstable();
    assert_eq!(call(vec![4,6,7,7]), result);

    assert_eq!(call(vec![4,4,3,2,1]), vec![vec![4,4]]);

    let result: Vec<Vec<_>> = Vec::new();
    assert_eq!(call(vec![4]), result);
}

fn call(nums: Vec<i32>) -> Vec<Vec<i32>> {
    Solution::find_subsequences(nums)
}
