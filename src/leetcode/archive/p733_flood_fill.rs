pub struct Solution;

impl Solution {
    pub fn flood_fill(mut image: Vec<Vec<i32>>, sr: i32, sc: i32, color: i32) -> Vec<Vec<i32>> {
        let (sr, sc) = (sr as usize, sc as usize);
        let from = image[sr][sc].clone();
        if from == color { return image; }
        Self::dfs(&mut image, sr, sc, from, color);
        image
    }

    fn dfs(mut image: &mut Vec<Vec<i32>>, r: usize, c: usize, from: i32, to: i32) {
        if image[r][c] == from {
            image[r][c] = to;
            if r > 0                    { Self::dfs(&mut image, r - 1, c, from, to); }
            if r < image.len() - 1      { Self::dfs(&mut image, r + 1, c, from, to); }
            if c > 0                    { Self::dfs(&mut image, r, c - 1, from, to); }
            if c < image[0].len() - 1   { Self::dfs(&mut image, r, c + 1, from, to); }
        }
    }
}
