/**
 * Given two binary strings a and b, return their sum as a binary string.
 * https://leetcode.com/problems/add-binary/submissions/865332996/
 *
 * Important! This solution is incorrect, even though it works. Because with realy big numbers it will fall down with overflow error.
 */
pub fn add_binary(a: String, b: String) -> String {
  let a_bin = u128::from_str_radix(&a, 2).expect("Not a binary number!");
  let b_bin = u128::from_str_radix(&b, 2).expect("Not a binary number!");
  return format!("{:b}", a_bin+b_bin);
}
