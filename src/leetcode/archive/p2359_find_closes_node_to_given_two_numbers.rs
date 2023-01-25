pub struct Solution;

impl Solution {
    pub fn closest_meeting_node(edges: Vec<i32>, node1: i32, node2: i32) -> i32 {
        let mut dist1 = vec![-1; edges.len()];
        let mut dist2 = vec![-1; edges.len()];

        Self::dfs(&mut dist1, &edges, node1 as usize, 0);
        Self::dfs(&mut dist2, &edges, node2 as usize, 0);

        let mut result = (i32::MAX, -1);
        for i in 0..edges.len() {
            if dist1[i] == -1 || dist2[i] == -1 { continue; }

            let cur = std::cmp::max(dist1[i], dist2[i]);
            if cur < result.0 {
                result = (cur, i as i32);
            }

        }

        result.1
    }

    fn dfs(mut dist: &mut Vec<i32>, edges: &Vec<i32>, node: usize, depth: i32) {
        if dist[node] != -1 { return; }
        dist[node] = depth;
        if edges[node] == -1 { return; }
        let node = edges[node] as usize;
        Self::dfs(&mut dist, &edges, node, depth + 1);
    }
}
