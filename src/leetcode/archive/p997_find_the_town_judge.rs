pub struct Solution;

impl Solution {
    pub fn find_judge(n: i32, trust: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let total = n * (n + 1) / 2;
        let mut sum_trust = vec![0; n as usize];
        let mut has_trust = vec![false; n as usize];
        for pair in trust {
            let (who, to_whom) = (pair[0] as usize, pair[1] as usize);
            sum_trust[to_whom - 1] += who;
            has_trust[who - 1] = true;
        }

        for i in 0..(n) {
            if has_trust[i] == false && sum_trust[i] == total - i - 1 {
                return (i + 1) as i32;
            }
        }

        return -1;
    }
}
