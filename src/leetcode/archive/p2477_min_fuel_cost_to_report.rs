pub struct Solution;

impl Solution {
    pub fn minimum_fuel_cost(roads: Vec<Vec<i32>>, seats: i32) -> i64 {
        let n = roads.len();

        if n == 0 {
            return 0;
        }

        let mut edges = vec![vec![]; n + 1];

        for road in roads {
            let (l, r) = (road[0] as usize, road[1] as usize);
            edges[r].push(l);
            edges[l].push(r);
        }

        let mut r: i64 = 0;
        Self::dfs(&edges, 0, 0, seats as i64, &mut r);

        r
    }

    fn dfs(edges: &Vec<Vec<usize>>, cur: usize, prev: usize, seats: i64,  mut r: &mut i64) -> i64 {
        let mut sum = 1;
        for &e in &edges[cur] {
            if e != prev {
                sum += Self::dfs(&edges, e, cur, seats, &mut r);
            }
        }

        if cur != 0 {
            *r += (sum-1) / seats + 1;
        }

        sum
    }
}
