use std::collections::HashSet;

pub struct Solution;

impl Solution {
    pub fn unique_paths_iii(grid: Vec<Vec<i32>>) -> i32 {
        let visits = HashSet::new();
        let mut zeros: usize = 0;
        let mut x: usize = 0;
        let mut y: usize = 0;
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if grid[i][j] == 0 {
                    zeros += 1;
                } else if grid[i][j] == 1 {
                    x = i;
                    y = j;
                }
            }
        }
        let mut result = 0;
        Self::r(&grid, visits.clone(), x, y, &zeros, &mut result);
        result
    }

    fn r(grid: &Vec<Vec<i32>>, mut visits: HashSet<(usize, usize)>, x: usize, y: usize, zeros: &usize, mut result: &mut i32) {
        if grid[x][y] == 2 {
            if visits.len() > *zeros {
                *result += 1;
            }

            return;
        }

        if visits.insert((x, y)) {
            if x > 0 && [0, 2].contains(&grid[x - 1][y]) {
                Self::r(&grid, visits.clone(), x - 1, y, &zeros, &mut result);
            }
            if x < grid.len() - 1 && [0, 2].contains(&grid[x + 1][y]) {
                Self::r(&grid, visits.clone(), x + 1, y, &zeros, &mut result);
            }
            if y > 0 && [0, 2].contains(&grid[x][y - 1]) {
                Self::r(&grid, visits.clone(), x, y - 1, &zeros, &mut result);
            }
            if y < grid[0].len() - 1 && [0, 2].contains(&grid[x][y + 1]) {
                Self::r(&grid, visits.clone(), x, y + 1, &zeros, &mut result);
            }
        }
    }
}
