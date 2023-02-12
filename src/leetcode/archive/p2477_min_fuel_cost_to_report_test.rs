use super::p2477_min_fuel_cost_to_report::Solution;

pub fn test() {
    assert_eq!(call(vec![vec![0,1],vec![0,2],vec![0,3]],5), 3);
    assert_eq!(call(vec![vec![3,1],vec![3,2],vec![1,0],vec![0,4],vec![0,5],vec![4,6]],2), 7);
    assert_eq!(call(vec![],1), 0);
}

fn call(roads: Vec<Vec<i32>>, seats: i32) -> i64 {
    Solution::minimum_fuel_cost(roads, seats)
}
