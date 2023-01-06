pub struct Solution;

impl Solution {
    pub fn minimum_rounds(tasks: Vec<i32>) -> i32 {
        let mut hm = std::collections::HashMap::new();
        for el in tasks {
            *hm.entry(el).or_insert(0) += 1;
        }

        let mut result = 0;
        for (_, el) in hm {
            if el==1 {
                return -1;
            }

            result += (el + 2) / 3;
        }

        result
    }
}