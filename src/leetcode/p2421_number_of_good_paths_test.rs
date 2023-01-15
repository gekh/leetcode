use super::p2421_number_of_good_paths::Solution;

pub fn test() {
    assert_eq!(
        call(
            vec![1, 3, 2, 1, 3],
            vec![vec![0, 1], vec![0, 2], vec![2, 3], vec![2, 4]]
        ),
        6
    );

    assert_eq!(
        call(
            vec![1, 1, 2, 2, 3],
            vec![vec![0, 1], vec![1, 2], vec![2, 3], vec![2, 4]]
        ),
        7
    );

    assert_eq!(
        call(
            vec![1, 1, 1, 1, 1],
            vec![vec![0, 1], vec![1, 2], vec![2, 3], vec![2, 4]]
        ),
        15
    );

    assert_eq!(call(vec![1], vec![]), 1);
    assert_eq!(call(vec![1,2], vec![vec![0,1]]), 2);
    assert_eq!(call(vec![1,1], vec![vec![0,1]]), 3);
}

fn call(vals: Vec<i32>, edges: Vec<Vec<i32>>) -> i32 {
    Solution::number_of_good_paths(vals, edges)
}
