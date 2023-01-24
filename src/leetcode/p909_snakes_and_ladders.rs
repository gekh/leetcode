pub struct Solution;

impl Solution {
    pub fn snakes_and_ladders(board: Vec<Vec<i32>>) -> i32 {
        let n = board.len();
        let mut flattened = vec![0];

        for i in 1..=(n * n) {
            let (x, y) = Self::convert_to_xy(n, i);
            flattened.push(board[x][y]);
        }

        let mut seen = vec![];
        let mut queue = std::collections::VecDeque::from([(1, 0)]);

        while let Some((mut cur, step)) = queue.pop_front() {
            if flattened[cur] != -1 {
                cur = flattened[cur] as usize;
            }

            if cur == n * n {
                return step;
            }

            for next in (cur + 1)..=(cur + 6).min(n * n) {
                if !seen.contains(&next) {
                    seen.push(next);
                    queue.push_back((next, step + 1));
                }
            }
        }

        -1
    }

    fn convert_to_xy(n: usize, mut position: usize) -> (usize, usize) {
        position -= 1;
        if position / n % 2 == 0 {
            (n - position / n - 1, position % n)
        } else {
            (n - position / n - 1, n - position % n - 1)
        }
    }
}
