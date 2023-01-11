pub struct Solution;

use std::collections::{HashMap, HashSet};
impl Solution {
    pub fn min_time(_n: i32, edges: Vec<Vec<i32>>, has_apple: Vec<bool>) -> i32 {
        let mut hm = HashMap::new();

        for pair in edges {
            let (a, b) = (pair[0], pair[1]);
            hm.entry(a).or_insert(vec![]).push(b);
            hm.entry(b).or_insert(vec![]).push(a);
        }

        let mut total = 0;
        let mut visited = HashSet::new();
        Self::dfs(0, &hm, &has_apple, &mut visited, &mut total, 0);

        total
    }

    fn dfs(
        cur: i32,
        hm: &HashMap<i32, Vec<i32>>,
        has_apple: &Vec<bool>,
        mut visited: &mut HashSet<i32>,
        mut total: &mut i32,
        depth: i32,
    ) -> bool {
        visited.insert(cur);

        match hm.get(&cur) {
            Some(children) => {
                let mut trues_count = 0;
                let do_has_apple = has_apple[cur as usize];

                for &child in children {
                    if visited.contains(&child) {
                        continue;
                    }
                    if Self::dfs(child, &hm, &has_apple, &mut visited, &mut total, depth + 2) {
                        trues_count += 1
                    }
                }

                if do_has_apple && trues_count == 0 {
                    *total += depth;
                } else if trues_count > 1 {
                    *total -= depth * (trues_count - 1);
                }

                do_has_apple || trues_count > 0
            }
            None => {
                if has_apple[cur as usize] {
                    *total += depth;
                    true
                } else {
                    false
                }
            }
        }
    }
}
