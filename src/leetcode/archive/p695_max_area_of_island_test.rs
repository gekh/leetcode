use super::p695_max_area_of_island::Solution;

pub fn test() {
    assert_eq!(
        call(vec![vec![0,0,1,0,0,0,0,1,0,0,0,0,0],vec![0,0,0,0,0,0,0,1,1,1,0,0,0],vec![0,1,1,0,1,0,0,0,0,0,0,0,0],vec![0,1,0,0,1,1,0,0,1,0,1,0,0],vec![0,1,0,0,1,1,0,0,1,1,1,0,0],vec![0,0,0,0,0,0,0,0,0,0,1,0,0],vec![0,0,0,0,0,0,0,1,1,1,0,0,0],vec![0,0,0,0,0,0,0,1,1,0,0,0,0]]),
        6
    );

    assert_eq!(
        call(vec![vec![0,0,0,0,0,0,0,0]]),
        0
    );
}

fn call(grid: Vec<Vec<i32>>) -> i32 {
    Solution::max_area_of_island(grid)

}
