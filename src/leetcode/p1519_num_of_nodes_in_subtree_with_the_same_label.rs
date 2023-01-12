pub struct Solution;

impl Solution {
    pub fn count_sub_trees(n: i32, edges: Vec<Vec<i32>>, labels: String) -> Vec<i32> {
        let mut tree = vec![Vec::new(); n as usize];
        for pair in edges {
            let (a, b) = (pair[0] as usize, pair[1] as usize);
            tree[a].push(b);
            tree[b].push(a);
        }

        let mut result = vec![0; n as usize];
        Self::dfs(!0 as usize, 0, &tree, labels.as_bytes(), &mut result);

        result
    }

    fn dfs(
        prev: usize,
        node: usize,
        tree: &Vec<Vec<usize>>,
        b_labels: &[u8],
        mut result: &mut Vec<i32>,
    ) -> [i32; 26] {
        let b_letter = (b_labels[node] - b'a') as usize;
        let mut counts = [0; 26];
        counts[b_letter] += 1;

        for &rel in tree[node].iter() {
            if rel != prev {
                let subcounts = Self::dfs(node, rel, &tree, &b_labels, &mut result);
                for (i, &v) in subcounts.iter().enumerate() {
                    if v > 0 {
                        counts[i] += v;
                    }
                }
            }
        }

        result[node] = counts[b_letter];

        counts
    }
}
