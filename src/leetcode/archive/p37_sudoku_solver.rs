pub struct Solution;

impl Solution {
    pub fn solve_sudoku(mut board: &mut Vec<Vec<char>>) {
        let mut rows = [[0; 9]; 9];
        let mut cols = [[0; 9]; 9];
        let mut groups = [[0; 9]; 9];

        for i in 0..9 {
            for j in 0..9 {
                if board[i][j] == '.' {
                    continue;
                }

                let n = (board[i][j] as u8 - b'0' - 1) as usize;
                rows[i][n] = 1;
                cols[j][n] = 1;
                let group_id = (i / 3) * 3 + j / 3;
                groups[group_id][n] = 1;
            }
        }

        let mut solved = false;
        Self::backtrack(0, 0, &mut board, &mut rows, &mut cols, &mut groups, &mut solved);
    }

    fn backtrack(
        i: usize,
        j: usize,
        mut board: &mut Vec<Vec<char>>,
        mut rows: &mut [[usize; 9]; 9],
        mut cols: &mut [[usize; 9]; 9],
        mut groups: &mut [[usize; 9]; 9],
        mut solved: &mut bool,
    ) {
        if i == 9 {
            *solved = true;
            return;
        }

        let ii = i + (j + 1) / 9;
        let jj = (j + 1) % 9;

        if board[i][j] != '.' {
            Self::backtrack(ii, jj, &mut board, &mut rows, &mut cols, &mut groups, &mut solved);
        } else {
            for k in 0..9 {
                let group_id = (i / 3) * 3 + j / 3;
                if rows[i][k] == 1 || cols[j][k] == 1 || groups[group_id][k] == 1 {
                    continue;
                }

                board[i][j] = (k as u8 + 1 + b'0') as char;
                rows[i][k] = 1;
                cols[j][k] = 1;
                groups[group_id][k] = 1;

                Self::backtrack(ii, jj, &mut board, &mut rows, &mut cols, &mut groups, &mut solved);

                if *solved == false {
                    board[i][j] = '.';
                    rows[i][k] = 0;
                    cols[j][k] = 0;
                    groups[group_id][k] = 0;
                }
            }
        }
    }
}
