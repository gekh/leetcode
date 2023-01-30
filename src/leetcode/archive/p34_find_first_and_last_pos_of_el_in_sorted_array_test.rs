use super::p34_find_first_and_last_pos_of_el_in_sorted_array::Solution;

pub fn test() {
    // assert_eq!(call(vec![5,7,7,8,8,10], 8), vec![3,4]);
    // assert_eq!(call(vec![5,7,7,8,8,10], 6), vec![-1,-1]);
    // assert_eq!(call(vec![], 0), vec![-1,-1]);
    // assert_eq!(call(vec![1], 1), vec![0,0]);
    assert_eq!(call(vec![1], 0), vec![-1,-1]);
}

fn call(nums: Vec<i32>, target: i32) -> Vec<i32> {
    Solution::search_range(nums, target)
}
