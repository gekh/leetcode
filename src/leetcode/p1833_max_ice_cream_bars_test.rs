use super::p1833_max_ice_cream_bars::Solution;

pub fn test() {
    assert_eq!(call(vec![1,3,2,4,1], 7), 4);
    assert_eq!(call(vec![10,6,8,7,7,8], 5), 0);
    assert_eq!(call(vec![1,6,3,1,2,5], 20), 6);
}

fn call(costs: Vec<i32>, coins: i32) -> i32 {
    Solution::max_ice_cream(costs, coins)
}
