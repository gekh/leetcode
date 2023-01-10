use super::p994_rotting_oranges::Solution;

pub fn test() {
    assert_eq!(call(vec![
        vec![2, 1, 1],
        vec![1, 1, 0],
        vec![0, 1, 1]]),
        4);

    assert_eq!(call(vec![vec![2, 1, 1], vec![1, 1, 0], vec![0, 2, 1]]), 2);

    assert_eq!(call(vec![vec![2, 1, 1], vec![0, 1, 1], vec![1, 0, 1]]), -1);

    assert_eq!(
        call(vec![
            vec![2, 1, 1, 1, 1],
            vec![0, 1, 1, 1, 1],
            vec![0, 0, 1, 2, 1]
        ]),
        3
    );

    assert_eq!(call(vec![vec![0, 2]]), 0);
    assert_eq!(call(vec![vec![0]]), 0);
    assert_eq!(call(vec![vec![1]]), -1);
    assert_eq!(call(vec![vec![2]]), 0);

    assert_eq!(
        call(vec![
            vec![2, 0, 1, 1, 1, 1, 1, 1, 1, 1],
            vec![1, 0, 1, 0, 0, 0, 0, 0, 0, 1],
            vec![1, 0, 1, 0, 1, 1, 1, 1, 0, 1],
            vec![1, 0, 1, 0, 1, 0, 0, 1, 0, 1],
            vec![1, 0, 1, 0, 1, 0, 0, 1, 0, 1],
            vec![1, 0, 1, 0, 1, 1, 0, 1, 0, 1],
            vec![1, 0, 1, 0, 0, 0, 0, 1, 0, 1],
            vec![1, 0, 1, 1, 1, 1, 1, 1, 0, 1],
            vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 1],
            vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1]
        ]),
        58
    );
}

fn call(grid: Vec<Vec<i32>>) -> i32 {
    Solution::oranges_rotting(grid)
}
