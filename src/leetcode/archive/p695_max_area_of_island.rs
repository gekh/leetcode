pub struct Solution;

impl Solution {
    pub fn max_area_of_island(mut grid: Vec<Vec<i32>>) -> i32 {
        let mut max = 0;
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if grid[i][j] == 1 {
                    max = max.max(Self::dfs(&mut grid, i, j));
                }
            }
        }
        max
    }

    fn dfs(mut image: &mut Vec<Vec<i32>>, r: usize, c: usize) -> i32 {
        let mut cnt = 0;
        if image[r][c] == 1 {
            image[r][c] = 2;
            cnt += 1;
            if r > 0                    { cnt += Self::dfs(&mut image, r - 1, c); }
            if r < image.len() - 1      { cnt += Self::dfs(&mut image, r + 1, c); }
            if c > 0                    { cnt += Self::dfs(&mut image, r, c - 1); }
            if c < image[0].len() - 1   { cnt += Self::dfs(&mut image, r, c + 1); }
        }
        return cnt;
    }
}
