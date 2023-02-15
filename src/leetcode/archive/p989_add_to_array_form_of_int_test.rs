use super::p989_add_to_array_form_of_int::Solution;

pub fn test() {
    assert_eq!(call(vec![1,2,0,0], 34), vec![1,2,3,4]);
    assert_eq!(call(vec![2,7,4], 181), vec![4,5,5]);
    assert_eq!(call(vec![2,7,4], 1810), vec![2,0,8,4]);
    assert_eq!(call(vec![2,1,5], 806), vec![1,0,2,1]);
    assert_eq!(call(vec![1], 5647), vec![5,6,4,8]);
    assert_eq!(call(vec![9,9,9,9,9,9,9,9,9,9], 1), vec![1, 0,0,0,0,0,0,0,0,0,0]);
}

fn call(num: Vec<i32>, k: i32) -> Vec<i32> {
    Solution::add_to_array_form(num, k)
}
