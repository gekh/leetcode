pub struct Solution;

impl Solution {
    pub fn add_to_array_form(num: Vec<i32>, k: i32) -> Vec<i32> {
        let k = k
            .to_string()
            .chars()
            .map(|x| x as i32 - b'0' as i32)
            .collect::<Vec<i32>>();

        let max_len = std::cmp::max(num.len(), k.len());
        let mut d = 0;
        let mut r = num
            .into_iter()
            .rev()
            .chain(std::iter::repeat(Default::default()))
            .zip(
                k.into_iter()
                    .rev()
                    .chain(std::iter::repeat(Default::default())),
            )
            .take(max_len)
            .map(|(x, y)| {
                let s = x + y + d;
                d = 0;
                if s >= 10 {
                    d = 1;
                    s - 10
                } else {
                    s
                }
            })
            .collect::<Vec<i32>>();

        if d > 0 {
            r.push(d);
        }
        r.reverse();
        r
    }
}
