use super::p189_rotate_array::Solution;

pub fn test() {
    let mut v = vec![1,2,3,4,5,6,7];
    call(&mut v, 3);
    assert!(v == vec![5,6,7,1,2,3,4]);

    let mut v = vec![1];
    call(&mut v, 0);
    assert!(v == vec![1]);

    let mut v = vec![1];
    call(&mut v, 1);
    assert!(v == vec![1]);

    let mut v = vec![1,2];
    call(&mut v, 5);
    assert!(v == vec![2,1]);
}

fn call(mut nums: &mut Vec<i32>, k: i32) {
    Solution::rotate(&mut nums, k)
}
