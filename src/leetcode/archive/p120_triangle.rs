pub struct Solution;

impl Solution {
    pub fn minimum_total(mut triangle: Vec<Vec<i32>>) -> i32 {
        let mut minlen = triangle.pop().unwrap();

        while let Some(row) = triangle.pop() {
            for i in 0..row.len() {
                minlen[i] = minlen[i].min(minlen[i + 1]) + row[i];
            }
        }

        minlen[0]
    }
}
