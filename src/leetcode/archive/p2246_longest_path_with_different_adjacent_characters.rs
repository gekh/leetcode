pub struct Solution;

impl Solution {
    pub fn longest_path(parent: Vec<i32>, s: String) -> i32 {
        let mut tree = vec![vec![]; parent.len()];

        for (child, &parent) in parent.iter().enumerate() {
            if parent == -1 {
                continue;
            }
            tree[parent as usize].push(child as usize);
        }

        let b_s = s.as_bytes().iter().map(|x| x - b'a').collect::<Vec<u8>>();
        let mut max = 1;
        Self::dfs(0, &tree, &b_s, &mut max);

        max
    }

    fn dfs(node: usize, tree: &Vec<Vec<usize>>, b_s: &Vec<u8>, mut max: &mut i32) -> (u8, i32) {
        let c = b_s[node];
        let mut result = (c, 1);
        let mut lengths = vec![];

        for &el in &tree[node] {
            let next = Self::dfs(el, &tree, &b_s, &mut max);
            if next.0 != c {
                lengths.push(next.1);
                result.1 = result.1.max(next.1 + 1);
            }
        }

        if lengths.len() > 1 {
            lengths.sort();
            lengths.reverse();

            if lengths[0] + lengths[1] + 1 > *max {
                *max = lengths[0] + lengths[1] + 1;
            }
        }

        if result.1 > *max {
            *max = result.1;
        }

        result
    }
}
