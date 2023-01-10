pub struct Solution;

impl Solution {
    pub fn oranges_rotting(mut grid: Vec<Vec<i32>>) -> i32 {
        let (rows, cols) = (grid.len(), grid[0].len());

        let mut fresh = 0;
        let mut dq = std::collections::VecDeque::new();

        for x in 0..rows {
            for y in 0..cols {
                match grid[x][y] {
                    1 => fresh += 1,
                    2 => dq.push_front((x,y)),
                    _ => (),
                };
            }
        }

        let mut minutes = 0;

        while fresh > 0 && dq.len() > 0 {
            // println!("{:?}", dq);
            minutes += 1;
            for _ in 0..dq.len() {
                if let Some((x, y)) = dq.pop_back() {
                    for coord in [0,1,0,-1,0].windows(2) {
                        let (dx, dy) = (coord[0], coord[1]);
                        if x as i32 + dx < 0 || x as i32 + dx >= rows as i32 { continue; }
                        if y as i32 + dy < 0 || y as i32 + dy >= cols as i32 { continue; }
                        let (xdx, ydy) = ((x as i32 + dx) as usize, (y as i32 + dy) as usize);

                        if grid[xdx][ydy] == 1 {
                            grid[xdx][ydy] = 2;
                            fresh -= 1;
                            dq.push_front((xdx, ydy));
                        }
                    }
                }
            }
        }

        if fresh > 0 {
            -1
        } else {
            minutes
        }
    }
}
