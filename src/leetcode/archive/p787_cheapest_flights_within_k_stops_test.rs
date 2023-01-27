use super::p787_cheapest_flights_within_k_stops::Solution;

pub fn test() {
    assert_eq!(call(4, vec![vec![0,1,100],vec![1,2,100],vec![2,0,100],vec![1,3,600],vec![2,3,200]], 0, 3, 1), 700);
    assert_eq!(call(4, vec![vec![0,1,100],vec![1,2,100],vec![2,0,100],vec![1,3,600],vec![2,3,200]], 0, 3, 2), 400);
    assert_eq!(call(3, vec![vec![0,1,100],vec![1,2,100],vec![0,2,500]], 0, 2, 1), 200);
    assert_eq!(call(3, vec![vec![0,1,100],vec![1,2,100],vec![0,2,500]], 0, 2, 0), 500);
    assert_eq!(call(4, vec![vec![0,1,100],vec![1,2,100],vec![0,2,500],vec![0,3,10], vec![3,1,10]], 0, 2, 2), 120);
    assert_eq!(call(1, vec![], 0, 0, 0), 0);
    assert_eq!(call(2, vec![], 0, 1, 1), -1);
    assert_eq!(call(2, vec![vec![1,0,5]], 0, 1, 1), -1);
    assert_eq!(call(10, vec![vec![3,4,4],vec![2,5,6],vec![4,7,10],vec![9,6,5],vec![7,4,4],vec![6,2,10],vec![6,8,6],vec![7,9,4],vec![1,5,4],vec![1,0,4],vec![9,7,3],vec![7,0,5],vec![6,5,8],vec![1,7,6],vec![4,0,9],vec![5,9,1],vec![8,7,3],vec![1,2,6],vec![4,1,5],vec![5,2,4],vec![1,9,1],vec![7,8,10],vec![0,4,2],vec![7,2,8]], 6, 0, 7), 14);
}

fn call(n: i32, flights: Vec<Vec<i32>>, src: i32, dst: i32, k: i32) -> i32 {
    Solution::find_cheapest_price(n, flights, src, dst, k)
}
