use super::p198_house_robber::Solution;

pub fn test() {
    assert_eq!(call(vec![2,9,1,3,8,4]), 17);
    assert_eq!(call(vec![7,9,3,1,9,7]), 19);
    assert_eq!(call(vec![1, 2, 1, 3]), 5);
    assert_eq!(call(vec![1, 2, 3, 1]), 4);
    assert_eq!(call(vec![2,7,9,3,1]), 12);
    assert_eq!(call(vec![7,9,1,1,9,7]), 18);
    assert_eq!(call(vec![7]), 7);
}

fn call(nums: Vec<i32>) -> i32 {
    Solution::rob(nums)
}
