pub struct Solution;
// https://stackoverflow.com/questions/60916194/how-to-sort-a-vector-in-descending-order-in-rust
impl Solution {
    pub fn max_ice_cream(mut costs: Vec<i32>, mut coins: i32) -> i32 {
        costs.sort_unstable_by_key(|&x| std::cmp::Reverse(x));

        let mut result = 0;
        while let Some(price) = costs.pop() {
            if price > coins { break; }
            result += 1;
            coins -= price;
        }

        result
    }
}
