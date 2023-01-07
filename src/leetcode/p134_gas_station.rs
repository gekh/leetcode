pub struct Solution;

impl Solution {
    pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
        let (mut start, mut tank, mut total) = (0,0,0);

        for i in 0..gas.len() {
            tank += gas[i] - cost[i];
            if tank < 0 {
                start = i + 1;
                total += tank;
                tank = 0;
            }
        }

        if total + tank < 0 { -1 } else { start as _ }
    }
}
