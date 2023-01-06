use super::p980_unique_paths_iii::Solution;

pub fn test() {
    let mut grid = vec![
        vec![1,0,0,0],
        vec![0,0,0,0],
        vec![0,0,2,-1],
    ];
    assert!(call(grid) == 2);

    grid = vec![
        vec![0,0,0,0,0,0,2,0,0,0],
        vec![0,0,0,0,0,0,0,0,1,0]
    ];
    assert!(call(grid) == 1);
}

fn call(grid: Vec<Vec<i32>>) -> i32 {
    Solution::unique_paths_iii(grid)
}
