use super::p57_insert_interval::Solution;

pub fn test() {
    assert_eq!( call(vec![vec![1, 3], vec![6, 9]], vec![2, 5]), vec![vec![1, 5], vec![6, 9]] );
    assert_eq!( call(vec![vec![1, 5], vec![6, 9]], vec![2, 3]), vec![vec![1, 5], vec![6, 9]] );
    assert_eq!( call(vec![vec![1, 5], vec![6, 9]], vec![1, 5]), vec![vec![1, 5], vec![6, 9]] );
    assert_eq!( call(vec![vec![2, 5], vec![6, 9]], vec![1, 3]), vec![vec![1, 5], vec![6, 9]] );
    assert_eq!( call(vec![vec![2, 3], vec![6, 9]], vec![1, 5]), vec![vec![1, 5], vec![6, 9]] );
    assert_eq!( call(vec![vec![2, 3], vec![6, 9]], vec![4, 5]), vec![vec![2, 3], vec![4, 5], vec![6, 9]] );
    assert_eq!( call(vec![vec![1, 1], vec![2, 3], vec![6, 9]], vec![4, 5]), vec![vec![1,1], vec![2, 3], vec![4, 5], vec![6, 9]] );
    assert_eq!( call(vec![ vec![1, 3], vec![4, 6], vec![7, 9], vec![10, 11] ], vec![2, 8]), vec![vec![1, 9], vec![10, 11]]);
    assert_eq!( call(vec![vec![1, 2], vec![6, 9]], vec![3, 5]), vec![vec![1, 2], vec![3, 5], vec![6, 9]] );
    assert_eq!( call( vec![ vec![1, 2], vec![3, 5], vec![6, 7], vec![8, 10], vec![12, 16] ], vec![4, 8] ), vec![vec![1, 2], vec![3, 10], vec![12, 16]] );
    assert_eq!( call(vec![], vec![3, 5]), vec![vec![3, 5]] );
    assert_eq!( call(vec![vec![1,4]], vec![3, 5]), vec![vec![1, 5]] );
    assert_eq!( call(vec![vec![1,2]], vec![3, 5]), vec![vec![1, 2], vec![3, 5]] );
    assert_eq!( call(vec![vec![1,5]], vec![2, 7]), vec![vec![1, 7]] );
    assert_eq!( call(vec![vec![1,5]], vec![0, 0]), vec![vec![0,0], vec![1, 5]] );
    assert_eq!( call(vec![vec![2,5]], vec![0, 1]), vec![vec![0,1], vec![2, 5]] );
}

fn call(intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
    Solution::insert(intervals, new_interval)
}
