use std::collections::BTreeSet;

pub struct SummaryRanges {
    nums: BTreeSet<i32>,
}

impl SummaryRanges {
    pub fn new() -> Self {
        Self {
            nums: BTreeSet::new(),
        }
    }

    pub fn add_num(&mut self, n: i32) {
        self.nums.insert(n);
    }

    pub fn get_intervals(&self) -> Vec<Vec<i32>> {

        if self.nums.len() == 0 {
            return vec![];
        }

        let mut result = vec![];
        let mut nums_iter = self.nums.iter();
        let mut cur = vec![*nums_iter.next().unwrap()];
        let mut prev = cur[0];
        for &n in nums_iter {
            if prev + 1 != n {
                cur.push(prev);
                result.push(cur);
                cur = vec![n];
            }

            prev = n;
        }

        cur.push(prev);
        result.push(cur);

        result
    }
}
