/**
 * Given a non-negative integer x, return the square root of x rounded down
 *  to the nearest integer.
 * https://leetcode.com/problems/sqrtx/
 */
pub fn my_sqrt(x: i32) -> i32 {
    let x = x as u64;
    let mut r: u64 = x / 2 + 1;
    loop {
        r = (r + x / r) / 2;

        if r.pow(2) <= x && (r + 1).pow(2) > x {
            return r as i32;
        }
    }
}
