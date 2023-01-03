use super::p977_squares_of_a_sorted_array::Solution;

pub fn test() {

    assert!(call(vec![-4,-1,0,3,10]) == vec![0,1,9,16,100]);
    assert!(call(vec![-7,-3,2,3,11]) == vec![4,9,9,49,121]);
    assert!(call(vec![-1,1]) == vec![1,1]);
    assert!(call(vec![1]) == vec![1]);
    assert!(call(vec![-1]) == vec![1]);

}

fn call(nums: Vec<i32>) -> Vec<i32> {
    Solution::sorted_squares(nums)
}
