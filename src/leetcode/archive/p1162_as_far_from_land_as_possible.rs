pub struct Solution;

use std::collections::HashSet;
impl Solution {
    pub fn max_distance(mut grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();

        let mut queue = HashSet::new();

        for i in 0..n {
            for j in 0..n {
                if grid[i][j] == 1 {
                    queue.insert((i,j));
                }
            }
        }

        let mut max = 0;

        while queue.len() > 0 {

            let mut new_queue = HashSet::new();
            for _ in 0..queue.len() {
                let el = queue.iter().next().cloned().unwrap();
                let (x,y) = queue.take(&el).unwrap();

                if x < n-1 {
                    if grid[x+1][y] == 0 {
                        grid[x+1][y] = grid[x][y] + 1;
                        new_queue.insert((x+1,y));
                        max = max.max(grid[x+1][y]);
                    }
                }

                if x > 0 {
                    if grid[x-1][y] == 0 {
                        grid[x-1][y] = grid[x][y] + 1;
                        new_queue.insert((x-1,y));
                        max = max.max(grid[x-1][y]);
                    }
                }

                if y < n - 1 {
                    if grid[x][y+1] == 0 {
                        grid[x][y+1] = grid[x][y] + 1;
                        new_queue.insert((x,y+1));
                        max = max.max(grid[x][y+1]);
                    }
                }

                if y > 0 {
                    if grid[x][y-1] == 0 {
                        grid[x][y-1] = grid[x][y] + 1;
                        new_queue.insert((x,y-1));
                        max = max.max(grid[x][y-1]);
                    }
                }
            }

            std::mem::swap(&mut queue, &mut new_queue);
        }

        max - 1
    }
}

