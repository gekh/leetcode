pub struct Solution;

use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn distinct_names(ideas: Vec<String>) -> i64 {
        let mut sets = HashMap::new();

        for i in ideas {
            let key = i[..1].to_string();
            sets.entry(key)
                .or_insert(HashSet::new())
                .insert(i[1..].to_string());
        }

        let keys: Vec<&String> = sets.keys().collect();

        let mut r = 0;
        for i in 0..(keys.len() - 1) {
            for j in (i + 1)..keys.len() {
                let a = sets.get(keys[i]).unwrap();
                let b = sets.get(keys[j]).unwrap();
                let intersetion = a.intersection(b).count();
                r += (a.len() - intersetion) * (b.len() - intersetion) * 2;
            }
        }

        r as _
    }
}
