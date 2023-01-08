use super::p149_max_points_on_a_line::Solution;

pub fn test() {
    assert_eq!(call(vec![vec![1, 1], vec![2, 2], vec![3, 3]]), 3);
    assert_eq!(call(vec![vec![0, 0]]), 1);
    assert_eq!( call(vec![ vec![1, 1], vec![3, 2], vec![5, 3], vec![4, 1], vec![2, 3], vec![1, 4] ]), 4 );
    assert_eq!( call(vec![ vec![0, 0], vec![4, 5], vec![7, 8], vec![8, 9], vec![5, 6], vec![3, 4], vec![1, 1] ]), 5 );

}

fn call(points: Vec<Vec<i32>>) -> i32 {
    Solution::max_points(points)
}
