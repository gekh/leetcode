pub struct Solution;

impl Solution {
    pub fn find_cheapest_price(n: i32, flights: Vec<Vec<i32>>, src: i32, dst: i32, k: i32) -> i32 {
        let (n,k) = (n as usize, k as usize);
        let mut cost = vec![i32::MAX; n];
        cost[src as usize] = 0;

        for _ in 0..(k+1) {
            let mut new_cost = cost.clone();
            for f in &flights {
                let (from, to) = (f[0] as usize, f[1] as usize);
                if cost[from] != i32::MAX {
                    new_cost[to] = new_cost[to].min(cost[from] + f[2]);
                }
            }
            cost = new_cost;
        }

        match cost[dst as usize] {
            i32::MAX => -1,
            x => x,
        }
    }
}