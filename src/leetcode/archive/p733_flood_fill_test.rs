use super::p733_flood_fill::Solution;

pub fn test() {
    assert_eq!(
        call(vec![vec![1, 1, 1], vec![1, 1, 0], vec![1, 0, 1]], 1, 1, 2),
        vec![vec![2, 2, 2], vec![2, 2, 0], vec![2, 0, 1]]
    );

    assert_eq!(
        call(vec![vec![0, 0, 0], vec![0, 0, 0]], 0, 0, 0),
        vec![vec![0, 0, 0], vec![0, 0, 0]]
    );
}

fn call(image: Vec<Vec<i32>>, sr: i32, sc: i32, color: i32) -> Vec<Vec<i32>> {
    Solution::flood_fill(image, sr, sc, color)
}
