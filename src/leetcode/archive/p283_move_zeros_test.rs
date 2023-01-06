use super::p283_move_zeros::Solution;

pub fn test() {
    let mut nums = vec![0,1,0,3,12];
    call(&mut nums);
    assert!(nums == vec![1,3,12,0,0]);
}

fn call(nums: &mut Vec<i32>) {
    Solution::move_zeroes(nums)
}
