pub struct Solution;

impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        nums.into_iter().fold((0, 0), |(pp,p), n|
            (p, std::cmp::max(n+pp, p))
        ).1
    }
}
