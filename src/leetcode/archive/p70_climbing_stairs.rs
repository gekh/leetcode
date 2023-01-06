pub struct Solution();

impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        (0..n).fold((0,1), |(a,b), _| (b, a+b)).1
    }
}
