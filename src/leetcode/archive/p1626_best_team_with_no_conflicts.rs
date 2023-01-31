pub struct Solution;

impl Solution {
    pub fn best_team_score(scores: Vec<i32>, ages: Vec<i32>) -> i32 {
        let mut data = vec![];
        let l = scores.len();

        for i in 0..l {
            data.push((scores[i], ages[i], i));
        }

        data.sort_unstable();

        let mut r: Vec<i32> = vec![];

        for (score, ..) in &data {
            r.push(*score);
        }

        for i in 1..l {
            let cur = r[i].clone();
            for j in 0..i {
                if data[j].1 <= data[i].1 {
                    r[i] = r[i].max(cur + r[j]);
                }
            }
        }

        *r.iter().max().unwrap()
    }
}
