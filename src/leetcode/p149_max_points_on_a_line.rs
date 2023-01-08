pub struct Solution;

impl Solution {
    pub fn max_points(points: Vec<Vec<i32>>) -> i32 {
        use std::collections::{HashMap, HashSet};

        let mut max = 1;
        let mut hm = HashMap::new();
        for i in 0..points.len() {
            for j in i + 1..points.len() {
                let dx = points[j][0] - points[i][0];
                let dy = points[j][1] - points[i][1];

                let formula = match (dx, dy) {
                    (0, 0) => continue,
                    (0, _) => format!("x = {}", points[i][0]),
                    (_, 0) => format!("y = {}", points[i][1]),
                    (_, _) => format!(
                        "{}x + {}",
                        dy as f32 / dx as f32,
                        -(dy as f32 / dx as f32) * points[j][0] as f32 + points[j][1] as f32
                    ),
                };

                let hm_entry = hm.entry(formula).or_insert(HashSet::new());
                hm_entry.insert(i);
                hm_entry.insert(j);
                if max < hm_entry.len() {
                    max = hm_entry.len();
                }
            }
        }

        max as _
    }
}
